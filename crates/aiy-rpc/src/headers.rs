/// Chain ID of the current chain
pub const X_AIY_CHAIN_ID: &str = "x-aiy-chain-id";

/// Chain name of the current chain
pub const X_AIY_CHAIN: &str = "x-aiy-chain";

/// Current checkpoint height
pub const X_AIY_CHECKPOINT_HEIGHT: &str = "x-aiy-checkpoint-height";

/// Lowest available checkpoint for which transaction and checkpoint data can be requested.
///
/// Specifically this is the lowest checkpoint for which the following data can be requested:
///  - checkpoints
///  - transactions
///  - effects
///  - events
pub const X_AIY_LOWEST_AVAILABLE_CHECKPOINT: &str = "x-aiy-lowest-available-checkpoint";

/// Lowest available checkpoint for which object data can be requested.
///
/// Specifically this is the lowest checkpoint for which input/output object data will be
/// available.
pub const X_AIY_LOWEST_AVAILABLE_CHECKPOINT_OBJECTS: &str =
    "x-aiy-lowest-available-checkpoint-objects";

/// Current epoch of the chain
pub const X_AIY_EPOCH: &str = "x-aiy-epoch";

/// Current timestamp of the chain - represented as number of milliseconds from the Unix epoch
pub const X_AIY_TIMESTAMP_MS: &str = "x-aiy-timestamp-ms";

/// Current timestamp of the chain - encoded in the [RFC 3339] format.
///
/// [RFC 3339]: https://www.ietf.org/rfc/rfc3339.txt
pub const X_AIY_TIMESTAMP: &str = "x-aiy-timestamp";
