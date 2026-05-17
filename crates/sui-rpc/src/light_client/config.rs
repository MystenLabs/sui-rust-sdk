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

    /// Maximum number of epochs the discovery walk is willing to
    /// advance through in a single call to the ratchet driver.
    ///
    /// Defaults to 10,000 — well past a year of testnet (~600 epochs)
    /// with significant margin. The guard exists so that a misbehaving
    /// (or malicious) server that keeps reporting `last_checkpoint <
    /// target_seq` cannot trap the client in an indefinite discovery
    /// loop. When the cap is hit the ratchet fails with
    /// [`crate::light_client::LightClientError::RatchetGapTooLarge`]
    /// rather than continuing to issue `GetEpoch` calls.
    pub max_ratchet_gap: u64,
}

impl Default for RatchetConfig {
    fn default() -> Self {
        Self {
            concurrency: 4,
            max_ratchet_gap: 10_000,
        }
    }
}
