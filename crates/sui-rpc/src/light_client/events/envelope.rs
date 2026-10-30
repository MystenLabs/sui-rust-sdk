//! SDK-side envelope for authenticated events received from the v2
//! `LedgerService.ListEvents` RPC filtered by `EventStreamHeadFilter`.

use sui_sdk_types::Digest;
use sui_sdk_types::Event;

use crate::field::FieldMask;
use crate::field::FieldMaskUtil;
use crate::proto::TryFromProtoError;
use crate::proto::sui::rpc::v2::Event as ProtoEvent;

/// A single authenticated event paired with the positional metadata a
/// verifier needs to reconstruct its `EventCommitment` leaf.
///
/// The tuple `(checkpoint, transaction_index, event_index)` identifies
/// the event's position in the ledger; combined with the per-event
/// digest derived from `event`, those four pieces form the BCS-encoded
/// merkle leaf the framework folds into the stream's MMR — see
/// `sui_sdk_types::framework::EventCommitment`.
///
/// `transaction_digest` is carried for caller convenience (e.g.,
/// correlating with an explorer URL or another transaction-keyed
/// lookup) but is not an input to the cryptographic verification.
///
/// The envelope deliberately omits any `stream_id`: every event in a
/// `ListEvents` stream filtered by `EventStreamHeadFilter` belongs to
/// the same stream by construction, and the caller already knows which
/// stream they asked for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthenticatedEvent {
    /// The checkpoint containing the transaction that emitted this event.
    pub checkpoint: u64,
    /// 0-based index of the emitting transaction within its containing
    /// checkpoint.
    pub transaction_index: u64,
    /// 0-based index of this event within its transaction's event list.
    pub event_index: u32,
    /// The digest of the emitting transaction.
    pub transaction_digest: Digest,
    /// The event payload itself.
    pub event: Event,
}

impl AuthenticatedEvent {
    /// Read mask selecting every proto `Event` field the
    /// [`TryFrom<&ProtoEvent>`] conversion requires: the ledger position,
    /// the emitting transaction's digest, and the event payload.
    pub(super) fn read_mask() -> FieldMask {
        FieldMask::from_paths([
            ProtoEvent::CHECKPOINT_FIELD.name,
            ProtoEvent::TRANSACTION_INDEX_FIELD.name,
            ProtoEvent::EVENT_INDEX_FIELD.name,
            ProtoEvent::TRANSACTION_DIGEST_FIELD.name,
            ProtoEvent::PACKAGE_ID_FIELD.name,
            ProtoEvent::MODULE_FIELD.name,
            ProtoEvent::SENDER_FIELD.name,
            ProtoEvent::EVENT_TYPE_FIELD.name,
            ProtoEvent::CONTENTS_FIELD.name,
        ])
    }
}

impl TryFrom<&ProtoEvent> for AuthenticatedEvent {
    type Error = TryFromProtoError;

    fn try_from(value: &ProtoEvent) -> Result<Self, Self::Error> {
        let checkpoint = value
            .checkpoint
            .ok_or_else(|| TryFromProtoError::missing(ProtoEvent::CHECKPOINT_FIELD.name))?;
        let transaction_index = value
            .transaction_index
            .ok_or_else(|| TryFromProtoError::missing(ProtoEvent::TRANSACTION_INDEX_FIELD.name))?;
        let event_index = value
            .event_index
            .ok_or_else(|| TryFromProtoError::missing(ProtoEvent::EVENT_INDEX_FIELD.name))?;
        let transaction_digest = value
            .transaction_digest
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing(ProtoEvent::TRANSACTION_DIGEST_FIELD.name))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(ProtoEvent::TRANSACTION_DIGEST_FIELD, e))?;
        let event = Event::try_from(value)?;

        Ok(Self {
            checkpoint,
            transaction_index,
            event_index,
            transaction_digest,
            event,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::FieldMaskTree;
    use crate::merge::Merge;
    use sui_sdk_types::Address;
    use sui_sdk_types::Identifier;
    use sui_sdk_types::StructTag;

    fn sample_authenticated_event() -> AuthenticatedEvent {
        AuthenticatedEvent {
            checkpoint: 42,
            transaction_index: 3,
            event_index: 7,
            transaction_digest: Digest::new([0xaa; 32]),
            event: Event {
                package_id: Address::TWO,
                module: Identifier::from_static("clock"),
                sender: Address::TWO,
                type_: StructTag::new(
                    Address::TWO,
                    Identifier::from_static("clock"),
                    Identifier::from_static("Tick"),
                    vec![],
                ),
                contents: vec![0xde, 0xad, 0xbe, 0xef],
            },
        }
    }

    /// The proto `Event` a server returns for `value` under
    /// [`AuthenticatedEvent::read_mask`].
    fn to_proto(value: &AuthenticatedEvent) -> ProtoEvent {
        ProtoEvent::from(value.event.clone())
            .with_checkpoint(value.checkpoint)
            .with_transaction_digest(value.transaction_digest.to_string())
            .with_transaction_index(value.transaction_index)
            .with_event_index(value.event_index)
    }

    #[test]
    fn round_trip_through_proto_preserves_envelope() {
        let original = sample_authenticated_event();
        let back = AuthenticatedEvent::try_from(&to_proto(&original)).unwrap();
        assert_eq!(back, original);
    }

    /// Every field the conversion reads is selected by the read mask,
    /// so a server honoring the mask returns a convertible event.
    #[test]
    fn read_mask_selects_every_converted_field() {
        let mask = FieldMaskTree::from(AuthenticatedEvent::read_mask());
        let masked = ProtoEvent::merge_from(&to_proto(&sample_authenticated_event()), &mask);
        assert_eq!(
            AuthenticatedEvent::try_from(&masked).unwrap(),
            sample_authenticated_event()
        );
    }

    #[test]
    fn missing_checkpoint_is_rejected() {
        let mut proto = to_proto(&sample_authenticated_event());
        proto.checkpoint = None;
        let err = AuthenticatedEvent::try_from(&proto).unwrap_err();
        assert_eq!(err.field_violation().field, "checkpoint");
    }

    #[test]
    fn missing_transaction_index_is_rejected() {
        let mut proto = to_proto(&sample_authenticated_event());
        proto.transaction_index = None;
        let err = AuthenticatedEvent::try_from(&proto).unwrap_err();
        assert_eq!(err.field_violation().field, "transaction_index");
    }

    #[test]
    fn missing_event_index_is_rejected() {
        let mut proto = to_proto(&sample_authenticated_event());
        proto.event_index = None;
        let err = AuthenticatedEvent::try_from(&proto).unwrap_err();
        assert_eq!(err.field_violation().field, "event_index");
    }

    #[test]
    fn missing_transaction_digest_is_rejected() {
        let mut proto = to_proto(&sample_authenticated_event());
        proto.transaction_digest = None;
        let err = AuthenticatedEvent::try_from(&proto).unwrap_err();
        assert_eq!(err.field_violation().field, "transaction_digest");
    }

    #[test]
    fn missing_payload_field_is_rejected() {
        let mut proto = to_proto(&sample_authenticated_event());
        proto.package_id = None;
        let err = AuthenticatedEvent::try_from(&proto).unwrap_err();
        assert_eq!(err.field_violation().field, "package_id");
    }

    /// A malformed `transaction_digest` (base58-undecodable) reports the
    /// `transaction_digest` field with a parse-error source.
    #[test]
    fn malformed_transaction_digest_is_rejected() {
        let mut proto = to_proto(&sample_authenticated_event());
        proto.transaction_digest = Some("not-a-real-digest!".into());
        let err = AuthenticatedEvent::try_from(&proto).unwrap_err();
        assert_eq!(err.field_violation().field, "transaction_digest");
    }
}
