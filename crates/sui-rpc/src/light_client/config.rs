//! Tunables for the light-client ratchet driver.

/// Configuration for the committee ratchet driver.
///
/// Constructed with [`Default`] and adjusted via the public fields, or
/// composed onto a [`crate::light_client::LightClient`] through its
/// `with_ratchet_config` builder.
#[derive(Debug, Clone)]
pub struct RatchetConfig {
    /// How many end-of-epoch checkpoint fetches to keep in flight at
    /// once. The discovery phase of the ratchet is inherently serial
    /// (each `GetEpoch` answer drives whether to walk to the next
    /// epoch), but once the set of epochs to advance through is known
    /// the end-of-epoch summaries can be fetched concurrently. The
    /// per-summary BLS verification still happens sequentially in
    /// epoch order — the cache is a state machine and updates must be
    /// ordered.
    ///
    /// Defaults to 4 — high enough to amortize round-trip latency for
    /// a fresh testnet bootstrap (~1000 epochs) without overwhelming a
    /// fullnode with parallel requests. Set to 1 to fall back to fully
    /// sequential behaviour.
    pub concurrency: usize,
}

impl Default for RatchetConfig {
    fn default() -> Self {
        Self { concurrency: 4 }
    }
}
