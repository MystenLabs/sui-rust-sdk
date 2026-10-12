//! Runtime shims, so the client runs on tokio natively and on the host's event loop on wasm32.
//!
//! On wasm32, futures that touch JavaScript (`fetch`, timers) are not `Send`, and there is no
//! tokio runtime to spawn tasks or drive timers. Everything here is a thin alias of tokio natively
//! and of the wasm32 equivalent otherwise, so the rest of the crate is written once.

#[cfg(not(target_arch = "wasm32"))]
mod imp {
    /// `Send` on every target except wasm32, where futures holding JavaScript values are not.
    pub trait MaybeSend: Send {}
    impl<T: Send + ?Sized> MaybeSend for T {}

    /// `Sync` on every target except wasm32.
    pub trait MaybeSync: Sync {}
    impl<T: Sync + ?Sized> MaybeSync for T {}
}

#[cfg(target_arch = "wasm32")]
mod imp {
    /// `Send` on every target except wasm32, where futures holding JavaScript values are not.
    pub trait MaybeSend {}
    impl<T: ?Sized> MaybeSend for T {}

    /// `Sync` on every target except wasm32.
    pub trait MaybeSync {}
    impl<T: ?Sized> MaybeSync for T {}
}

pub use imp::MaybeSend;
pub use imp::MaybeSync;

/// A boxed future, `Send` except on wasm32.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) type BoxFuture<'a, T> = futures::future::BoxFuture<'a, T>;
#[cfg(target_arch = "wasm32")]
pub(crate) type BoxFuture<'a, T> = futures::future::LocalBoxFuture<'a, T>;

/// A boxed stream of RPC results, `Send` except on wasm32.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) type BoxStream<T> = tonic::codegen::BoxStream<T>;
#[cfg(target_arch = "wasm32")]
pub(crate) type BoxStream<T> =
    std::pin::Pin<Box<dyn futures::Stream<Item = Result<T, tonic::Status>> + 'static>>;

/// A boxed [`tower::Service`], `Send` except on wasm32.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) type BoxService<T, U, E> = tower::util::BoxService<T, U, E>;
#[cfg(target_arch = "wasm32")]
pub(crate) type BoxService<T, U, E> = tower::util::UnsyncBoxService<T, U, E>;

/// Timers: tokio's natively, the host's (`setTimeout`) on wasm32.
pub(crate) mod time {
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) use tokio::time::Instant;
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) use tokio::time::Sleep;
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) use tokio::time::sleep;
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) use tokio::time::timeout;
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) use tokio::time::timeout_at;
    #[cfg(target_arch = "wasm32")]
    pub(crate) use wasmtimer::std::Instant;
    #[cfg(target_arch = "wasm32")]
    pub(crate) use wasmtimer::tokio::Sleep;
    #[cfg(target_arch = "wasm32")]
    pub(crate) use wasmtimer::tokio::sleep;
    #[cfg(target_arch = "wasm32")]
    pub(crate) use wasmtimer::tokio::timeout;
    #[cfg(target_arch = "wasm32")]
    pub(crate) use wasmtimer::tokio::timeout_at;
}

/// A spawned task that can be aborted and awaited for completion.
pub(crate) struct Task {
    #[cfg(not(target_arch = "wasm32"))]
    handle: tokio::task::JoinHandle<()>,
    /// Dropping the remote handle cancels the task. `RemoteHandle` panics if its `Remote` is
    /// dropped before finishing, which `spawn_local` never does: it drops a future only once
    /// it completes.
    #[cfg(target_arch = "wasm32")]
    handle: Option<futures::future::RemoteHandle<()>>,
}

/// How a [`Task`] ended.
pub(crate) enum TaskExit {
    /// It ran to completion or was aborted.
    Finished,
    /// It panicked (only observable natively; a wasm32 panic aborts the module).
    #[cfg_attr(target_arch = "wasm32", allow(dead_code))]
    Panicked,
}

impl Task {
    /// Spawns `future` on tokio natively, or on the host's event loop on wasm32.
    pub(crate) fn spawn<F>(future: F) -> Self
    where
        F: std::future::Future<Output = ()> + MaybeSend + 'static,
    {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Self {
                handle: tokio::spawn(future),
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            use futures::FutureExt as _;
            let (remote, handle) = future.remote_handle();
            wasm_bindgen_futures::spawn_local(remote);
            Self {
                handle: Some(handle),
            }
        }
    }

    /// Stops the task at its next await point.
    pub(crate) fn abort(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        self.handle.abort();
        #[cfg(target_arch = "wasm32")]
        drop(self.handle.take());
    }

    /// Polls for the task to end.
    pub(crate) fn poll_exit(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<TaskExit> {
        use std::future::Future as _;
        use std::pin::Pin;
        use std::task::Poll;

        #[cfg(not(target_arch = "wasm32"))]
        {
            match Pin::new(&mut self.handle).poll(cx) {
                Poll::Ready(Err(e)) if e.is_panic() => Poll::Ready(TaskExit::Panicked),
                // A clean exit, or cancellation by `abort`.
                Poll::Ready(_) => Poll::Ready(TaskExit::Finished),
                Poll::Pending => Poll::Pending,
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            match &mut self.handle {
                Some(handle) => Pin::new(handle).poll(cx).map(|()| TaskExit::Finished),
                None => Poll::Ready(TaskExit::Finished),
            }
        }
    }
}

/// Spawns `future` and lets it run to completion on its own.
#[cfg(feature = "unstable")]
pub(crate) fn spawn_detached<F>(future: F)
where
    F: std::future::Future<Output = ()> + MaybeSend + 'static,
{
    #[cfg(not(target_arch = "wasm32"))]
    {
        tokio::spawn(future);
    }
    #[cfg(target_arch = "wasm32")]
    {
        wasm_bindgen_futures::spawn_local(future);
    }
}
