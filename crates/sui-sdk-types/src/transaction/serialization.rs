use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde_with::DeserializeAs;
use serde_with::SerializeAs;

use crate::Address;
use crate::ObjectReference;

mod transaction {
    use super::*;
    use crate::Address;
    use crate::transaction::GasPayment;
    use crate::transaction::Transaction;
    use crate::transaction::TransactionExpiration;
    use crate::transaction::TransactionKind;

    #[derive(serde_derive::Serialize)]
    #[serde(rename = "Transaction")]
    enum TransactionDataRef<'a> {
        V1(TransactionV1Ref<'a>),
    }

    #[derive(serde_derive::Deserialize)]
    #[serde(rename = "Transaction")]
    enum TransactionData {
        V1(TransactionV1),
    }

    #[derive(serde_derive::Serialize)]
    #[serde(rename = "TransactionV1")]
    struct TransactionV1Ref<'a> {
        kind: &'a TransactionKind,
        sender: &'a Address,
        gas_payment: &'a GasPayment,
        expiration: &'a TransactionExpiration,
    }

    #[derive(serde_derive::Deserialize)]
    #[serde(rename = "TransactionV1")]
    struct TransactionV1 {
        kind: TransactionKind,
        sender: Address,
        gas_payment: GasPayment,
        expiration: TransactionExpiration,
    }

    impl Serialize for Transaction {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let transaction = TransactionV1Ref {
                kind: &self.kind,
                sender: &self.sender,
                gas_payment: &self.gas_payment,
                expiration: &self.expiration,
            };

            TransactionDataRef::V1(transaction).serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for Transaction {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let TransactionData::V1(TransactionV1 {
                kind,
                sender,
                gas_payment,
                expiration,
            }) = Deserialize::deserialize(deserializer)?;

            Ok(Transaction {
                kind,
                sender,
                gas_payment,
                expiration,
            })
        }
    }
}

mod input_argument {
    use crate::transaction::Input;

    use super::*;

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    enum CallArg {
        Pure(#[serde(with = "::serde_with::As::<::serde_with::Bytes>")] Vec<u8>),
        Object(ObjectArg),
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    enum ObjectArg {
        ImmutableOrOwned(ObjectReference),
        Shared {
            object_id: Address,
            initial_shared_version: u64,
            mutable: bool,
        },
        Receiving(ObjectReference),
    }

    impl Serialize for Input {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let binary = match self.clone() {
                Input::Pure { value } => CallArg::Pure(value),
                Input::ImmutableOrOwned(object_ref) => {
                    CallArg::Object(ObjectArg::ImmutableOrOwned(object_ref))
                }
                Input::Shared {
                    object_id,
                    initial_shared_version,
                    mutable,
                } => CallArg::Object(ObjectArg::Shared {
                    object_id,
                    initial_shared_version,
                    mutable,
                }),
                Input::Receiving(object_ref) => CallArg::Object(ObjectArg::Receiving(object_ref)),
            };
            binary.serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for Input {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            CallArg::deserialize(deserializer).map(|binary| match binary {
                CallArg::Pure(value) => Input::Pure { value },
                CallArg::Object(ObjectArg::ImmutableOrOwned(object_ref)) => {
                    Input::ImmutableOrOwned(object_ref)
                }
                CallArg::Object(ObjectArg::Shared {
                    object_id,
                    initial_shared_version,
                    mutable,
                }) => Input::Shared {
                    object_id,
                    initial_shared_version,
                    mutable,
                },
                CallArg::Object(ObjectArg::Receiving(object_ref)) => Input::Receiving(object_ref),
            })
        }
    }
}

pub(crate) use signed_transaction::SignedTransactionWithIntentMessage;

mod signed_transaction {
    use serde::ser::SerializeSeq;

    use super::*;
    use crate::UserSignature;
    use crate::transaction::SignedTransaction;
    use crate::transaction::Transaction;

    /// serde implementation that serializes a transaction prefixed with the signing intent. See
    /// [struct Intent] for more info.
    ///
    /// So we need to serialize Transaction as (0, 0, 0, Transaction)
    struct IntentMessageWrappedTransaction;

    impl SerializeAs<Transaction> for IntentMessageWrappedTransaction {
        fn serialize_as<S>(transaction: &Transaction, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            use serde::ser::SerializeTuple;

            let mut s = serializer.serialize_tuple(4)?;
            s.serialize_element(&0u8)?;
            s.serialize_element(&0u8)?;
            s.serialize_element(&0u8)?;
            s.serialize_element(transaction)?;
            s.end()
        }
    }

    impl<'de> DeserializeAs<'de, Transaction> for IntentMessageWrappedTransaction {
        fn deserialize_as<D>(deserializer: D) -> Result<Transaction, D::Error>
        where
            D: Deserializer<'de>,
        {
            let (scope, version, app, transaction): (u8, u8, u8, Transaction) =
                Deserialize::deserialize(deserializer)?;
            match (scope, version, app) {
                (0, 0, 0) => {}
                _ => {
                    return Err(serde::de::Error::custom(format!(
                        "invalid intent message ({scope}, {version}, {app})"
                    )));
                }
            }

            Ok(transaction)
        }
    }

    pub(crate) struct SignedTransactionWithIntentMessage;

    #[derive(serde_derive::Serialize)]
    struct BinarySignedTransactionWithIntentMessageRef<'a> {
        #[serde(with = "::serde_with::As::<IntentMessageWrappedTransaction>")]
        transaction: &'a Transaction,
        signatures: &'a Vec<UserSignature>,
    }

    #[derive(serde_derive::Deserialize)]
    struct BinarySignedTransactionWithIntentMessage {
        #[serde(with = "::serde_with::As::<IntentMessageWrappedTransaction>")]
        transaction: Transaction,
        signatures: Vec<UserSignature>,
    }

    impl SerializeAs<SignedTransaction> for SignedTransactionWithIntentMessage {
        fn serialize_as<S>(
            transaction: &SignedTransaction,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                transaction.serialize(serializer)
            } else {
                let SignedTransaction {
                    transaction,
                    signatures,
                } = transaction;
                let binary = BinarySignedTransactionWithIntentMessageRef {
                    transaction,
                    signatures,
                };

                let mut s = serializer.serialize_seq(Some(1))?;
                s.serialize_element(&binary)?;
                s.end()
            }
        }
    }

    impl<'de> DeserializeAs<'de, SignedTransaction> for SignedTransactionWithIntentMessage {
        fn deserialize_as<D>(deserializer: D) -> Result<SignedTransaction, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                SignedTransaction::deserialize(deserializer)
            } else {
                struct V;
                impl<'de> serde::de::Visitor<'de> for V {
                    type Value = SignedTransaction;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("expected a sequence with length 1")
                    }

                    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::SeqAccess<'de>,
                    {
                        if seq.size_hint().is_some_and(|size| size != 1) {
                            return Err(serde::de::Error::custom(
                                "expected a sequence with length 1",
                            ));
                        }

                        let BinarySignedTransactionWithIntentMessage {
                            transaction,
                            signatures,
                        } = seq.next_element()?.ok_or_else(|| {
                            serde::de::Error::custom("expected a sequence with length 1")
                        })?;
                        Ok(SignedTransaction {
                            transaction,
                            signatures,
                        })
                    }
                }

                deserializer.deserialize_seq(V)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use base64ct::Base64;
    use base64ct::Encoding;

    use crate::transaction::Transaction;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    #[test]
    fn transaction_fixtures() {
        const GENESIS_TRANSACTION: &str = include_str!("fixtures/genesis-transaction");
        const CONSENSUS_PROLOGUE: &str = "AAMAAAAAAAAAAAIAAAAAAAAAtkjHeocBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAIAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAAAAAAAAAAAAAAAAAAAAA==";
        const EPOCH_CHANGE: &str = "AAUCAmkBAAAAAAAAmSrgAQAAAAAAagEAAAAAAAApAAAAAAAAALAQCoNLLwAAnNn0sywGAABsVBEfSC0AAKQnlhd1AAAAzve+vo4BAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAA=";
        const AUTHENTICATOR_STATE_UPDATE: &str =
            include_str!("fixtures/authenticator_state_update");
        const PTB: &str = "AAADAQFEBbUNeR/TNGdU6Bcaqra8LtJsLEbv3QM8FLMK5QesMyx96QEAAAAAAQAIVsakAAAAAAABALyyokbZ/8ynfWQer6UyP1DpeCnPU1NC7AyFNJSaTztnQF40BQAAAAAgffPXh5XuG6TWjHk6qC5w9k2a+41oTWfm0sC1FOYRqsEBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAN7pB2Nsb2JfdjIMY2FuY2VsX29yZGVyAgcAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAgNzdWkDU1VJAAddSzAlBmRcN/8TO5jEtQpa4UhBZZc41tcz1Z0NIXqTvwRjb2luBENPSU4AAwEAAAEBAAECAPgh00g/x3Jeuvqlo9Ejc9SZAb384UhPIZ2qcGajDfd9ASXQjpFOD6mfycbzwD1wc+IOkCXQ8rHQo/Vi5SDOGMR/Jl40BQAAAAAgV7P1E0IMKon5uI82R/0arWLt+dc1ng/4VwKDqpTCxHT4IdNIP8dyXrr6paPRI3PUmQG9/OFITyGdqnBmow33fe4CAAAAAAAAAMqaOwAAAAAA";
        const WORMHOLE_PYTH_TRANSACTION: &str = include_str!("fixtures/wormhole-pyth-transaction");

        for fixture in [
            GENESIS_TRANSACTION,
            CONSENSUS_PROLOGUE,
            EPOCH_CHANGE,
            AUTHENTICATOR_STATE_UPDATE,
            PTB,
            WORMHOLE_PYTH_TRANSACTION,
        ] {
            let fixture = Base64::decode_vec(fixture.trim()).unwrap();
            let tx: Transaction = bcs::from_bytes(&fixture).unwrap();
            assert_eq!(bcs::to_bytes(&tx).unwrap(), fixture);

            let json = serde_json::to_string_pretty(&tx).unwrap();
            println!("{json}");
            assert_eq!(tx, serde_json::from_str(&json).unwrap());
        }
    }
}
