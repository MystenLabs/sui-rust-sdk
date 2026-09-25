/// Chain ID of the current chain
pub const X_SUI_CHAIN_ID: &str = "x-sui-chain-id";

/// Chain name of the current chain
pub const X_SUI_CHAIN: &str = "x-sui-chain";

/// Current checkpoint height
pub const X_SUI_CHECKPOINT_HEIGHT: &str = "x-sui-checkpoint-height";

/// Lowest available checkpoint for which transaction and checkpoint data can be requested.
///
/// Specifically this is the lowest checkpoint for which the following data can be requested:
///  - checkpoints
///  - transactions
///  - effects
///  - events
pub const X_SUI_LOWEST_AVAILABLE_CHECKPOINT: &str = "x-sui-lowest-available-checkpoint";

/// Lowest available checkpoint for which object data can be requested.
///
/// Specifically this is the lowest checkpoint for which input/output object data will be
/// available.
pub const X_SUI_LOWEST_AVAILABLE_CHECKPOINT_OBJECTS: &str =
    "x-sui-lowest-available-checkpoint-objects";

/// Current epoch of the chain
pub const X_SUI_EPOCH: &str = "x-sui-epoch";

/// Current timestamp of the chain - represented as number of milliseconds from the Unix epoch
pub const X_SUI_TIMESTAMP_MS: &str = "x-sui-timestamp-ms";

/// Current timestamp of the chain - encoded in the [RFC 3339] format.
///
/// [RFC 3339]: https://www.ietf.org/rfc/rfc3339.txt
pub const X_SUI_TIMESTAMP: &str = "x-sui-timestamp";

/// Request header carrying the highest protocol version whose types the client can decode.
///
/// The server may use it to avoid sending data the client cannot decode.
pub const X_SUI_CLIENT_PROTOCOL_VERSION: &str = "x-sui-client-protocol-version";

/// Highest Sui protocol version whose on-chain types this SDK can decode, sent as
/// [`X_SUI_CLIENT_PROTOCOL_VERSION`].
///
/// Bump this when the SDK gains support for types introduced in a newer protocol version.
pub const MAX_PROTOCOL_VERSION: u64 = 138;
