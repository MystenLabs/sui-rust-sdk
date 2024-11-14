mod v1;
mod v2;

pub use v1::ModifiedAtVersion;
pub use v1::ObjectReferenceWithOwner;
pub use v1::TransactionEffectsV1;
pub use v2::ChangedObject;
pub use v2::EffectsObjectChange;
pub use v2::IdOperation;
pub use v2::ObjectIn;
pub use v2::ObjectOut;
pub use v2::TransactionEffectsV2;
pub use v2::UnchangedSharedKind;
pub use v2::UnchangedSharedObject;

use crate::types::execution_status::ExecutionStatus;

/// The response from processing a transaction or a certified transaction
#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(
    feature = "schemars",
    derive(schemars::JsonSchema),
    schemars(tag = "version")
)]
#[cfg_attr(test, derive(test_strategy::Arbitrary))]
pub enum TransactionEffects {
    #[cfg_attr(feature = "schemars", schemars(rename = "1"))]
    V1(Box<TransactionEffectsV1>),
    #[cfg_attr(feature = "schemars", schemars(rename = "2"))]
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
            TransactionEffects::V1(e) => *e.epoch(),
            TransactionEffects::V2(e) => *e.epoch(),
        }
    }

    /// Return the gas cost summary of the transaction.
    pub fn gas_summary(&self) -> &crate::types::gas::GasCostSummary {
        match self {
            TransactionEffects::V1(e) => e.gas_summary(),
            TransactionEffects::V2(e) => e.gas_summary(),
        }
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
mod serialization {
    use super::TransactionEffects;
    use super::TransactionEffectsV1;
    use super::TransactionEffectsV2;

    use serde::Deserialize;
    use serde::Deserializer;
    use serde::Serialize;
    use serde::Serializer;

    #[derive(serde_derive::Serialize)]
    #[serde(tag = "version")]
    enum ReadableEffectsRef<'a> {
        #[serde(rename = "1")]
        V1(&'a TransactionEffectsV1),
        #[serde(rename = "2")]
        V2(&'a TransactionEffectsV2),
    }

    #[derive(serde_derive::Deserialize)]
    #[serde(tag = "version")]
    pub enum ReadableEffects {
        #[serde(rename = "1")]
        V1(Box<TransactionEffectsV1>),
        #[serde(rename = "2")]
        V2(Box<TransactionEffectsV2>),
    }

    #[derive(serde_derive::Serialize)]
    enum BinaryEffectsRef<'a> {
        V1(&'a TransactionEffectsV1),
        V2(&'a TransactionEffectsV2),
    }

    #[derive(serde_derive::Deserialize)]
    pub enum BinaryEffects {
        V1(Box<TransactionEffectsV1>),
        V2(Box<TransactionEffectsV2>),
    }

    impl Serialize for TransactionEffects {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                let readable = match self {
                    TransactionEffects::V1(fx) => ReadableEffectsRef::V1(fx),
                    TransactionEffects::V2(fx) => ReadableEffectsRef::V2(fx),
                };
                readable.serialize(serializer)
            } else {
                let binary = match self {
                    TransactionEffects::V1(fx) => BinaryEffectsRef::V1(fx),
                    TransactionEffects::V2(fx) => BinaryEffectsRef::V2(fx),
                };
                binary.serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for TransactionEffects {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                ReadableEffects::deserialize(deserializer).map(|readable| match readable {
                    ReadableEffects::V1(fx) => Self::V1(fx),
                    ReadableEffects::V2(fx) => Self::V2(fx),
                })
            } else {
                BinaryEffects::deserialize(deserializer).map(|binary| match binary {
                    BinaryEffects::V1(fx) => Self::V1(fx),
                    BinaryEffects::V2(fx) => Self::V2(fx),
                })
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
}
