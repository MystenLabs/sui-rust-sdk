use sui_sdk_types::*;
use test_strategy::proptest;

macro_rules! protobuf_roundtrip_test {
    ($type:ident, $proto:ty) => {
        paste::item! {
            #[proptest]
            #[allow(non_snake_case)]
            fn [< test_protobuf_roundtrip_ $type >] (instance: $type) {
                assert_roundtrip::<$type, $proto>(instance);
            }
        }
    };
}

/// Test that a type `T` can be roundtripped through a protobuf type `P`
fn assert_roundtrip<T, P>(instance: T)
where
    T: PartialEq + std::fmt::Debug + Clone,
    T: for<'a> TryFrom<&'a P, Error: std::fmt::Debug>,
    P: From<T>,
    P: prost::Message + Default,
    P: serde::Serialize + for<'de> serde::Deserialize<'de> + PartialEq + std::fmt::Debug,
{
    let proto = P::from(instance.to_owned());

    let proto_bytes = proto.encode_to_vec();

    let deser_from_proto = P::decode(proto_bytes.as_slice()).unwrap();

    let t_from_p = T::try_from(&deser_from_proto).unwrap();

    assert_eq!(instance, t_from_p);

    // Ensure that the protos can be roundtripped to json
    let json = serde_json::to_string(&proto).unwrap();
    let deser_from_json = serde_json::from_str::<P>(&json).unwrap();
    assert_eq!(proto, deser_from_json);
}

protobuf_roundtrip_test!(CheckpointSummary, super::CheckpointSummary);
protobuf_roundtrip_test!(CheckpointContents, super::CheckpointContents);
protobuf_roundtrip_test!(Transaction, super::Transaction);
protobuf_roundtrip_test!(TransactionEffects, super::TransactionEffects);
protobuf_roundtrip_test!(ChangedObject, super::ChangedObject);
protobuf_roundtrip_test!(UnchangedSharedObject, super::UnchangedSharedObject);
protobuf_roundtrip_test!(TransactionEvents, super::TransactionEvents);
protobuf_roundtrip_test!(Object, super::Object);
protobuf_roundtrip_test!(UserSignature, super::UserSignature);
protobuf_roundtrip_test!(
    ValidatorAggregatedSignature,
    super::ValidatorAggregatedSignature
);
protobuf_roundtrip_test!(ExecutionStatus, super::ExecutionStatus);

#[proptest]
fn test_protobuf_roundtrip_transaction_no_bcs(instance: Transaction) {
    use prost::Message;

    let mut proto = super::Transaction::from(instance.to_owned());
    proto.bcs = None; // Force BCS to be removed

    let proto_bytes = proto.encode_to_vec();

    let deser_from_proto = super::Transaction::decode(proto_bytes.as_slice()).unwrap();

    let t_from_p = Transaction::try_from(&deser_from_proto).unwrap();

    assert_eq!(instance, t_from_p);

    // Ensure that the protos can be roundtripped to json
    let json = serde_json::to_string(&proto).unwrap();
    let deser_from_json = serde_json::from_str::<super::Transaction>(&json).unwrap();
    assert_eq!(proto, deser_from_json);
}
