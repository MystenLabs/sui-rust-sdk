mod v1;
mod v2;

pub use v1::ModifiedAtVersion;
pub use v1::ObjectReferenceWithOwner;
pub use v1::TransactionEffectsV1;
pub use v2::ChangedObject;
pub use v2::IdOperation;
pub use v2::ObjectIn;
pub use v2::ObjectOut;
pub use v2::TransactionEffectsV2;
pub use v2::UnchangedSharedKind;
pub use v2::UnchangedSharedObject;

use crate::execution_status::ExecutionStatus;

/// The output or effects of executing a transaction
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// transaction-effects =  %x00 effects-v1
///                     =/ %x01 effects-v2
/// ```
#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub enum TransactionEffects {
    V1(Box<TransactionEffectsV1>),
    V2(Box<TransactionEffectsV2>),
}

impl TransactionEffects {
    /// Return the status of the transaction.
    pub fn status(&self) -> &ExecutionStatus {
        match self {
            TransactionEffects::V1(e) => e.status(),
            TransactionEffects::V2(e) => e.status(),
        }
    }

    /// Return the epoch in which this transaction was executed.
    pub fn epoch(&self) -> u64 {
        match self {
            TransactionEffects::V1(e) => e.epoch(),
            TransactionEffects::V2(e) => e.epoch(),
        }
    }

    /// Return the gas cost summary of the transaction.
    pub fn gas_summary(&self) -> &crate::gas::GasCostSummary {
        match self {
            TransactionEffects::V1(e) => e.gas_summary(),
            TransactionEffects::V2(e) => e.gas_summary(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TransactionEffects;

    use base64ct::Base64;
    use base64ct::Encoding;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    #[test]
    fn effects_fixtures() {
        const GENESIS_EFFECTS: &str = include_str!("fixtures/genesis-transaction-effects");
        const PYTH_WORMHOLE_V2: &str = include_str!("fixtures/pyth-wormhole-v2");

        for fixture in [GENESIS_EFFECTS, PYTH_WORMHOLE_V2] {
            let fixture = Base64::decode_vec(fixture.trim()).unwrap();
            let fx: TransactionEffects = bcs::from_bytes(&fixture).unwrap();
            assert_eq!(bcs::to_bytes(&fx).unwrap(), fixture);

            let json = serde_json::to_string_pretty(&fx).unwrap();
            println!("{json}");
            assert_eq!(fx, serde_json::from_str(&json).unwrap());
        }
    }
}
