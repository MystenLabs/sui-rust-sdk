//! SDK-side envelope for authenticated events received via the v2alpha
//! `LedgerService.ListEvents` RPC filtered by `EventStreamHeadFilter`.

use sui_sdk_types::Digest;
use sui_sdk_types::Event;

use crate::proto::TryFromProtoError;
use crate::proto::sui::rpc::v2alpha::EventItem;

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
/// `ListEvents` response filtered by `EventStreamHeadFilter` belongs to
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

impl TryFrom<&EventItem> for AuthenticatedEvent {
    type Error = TryFromProtoError;

    fn try_from(value: &EventItem) -> Result<Self, Self::Error> {
        let checkpoint = value
            .checkpoint
            .ok_or_else(|| TryFromProtoError::missing(EventItem::CHECKPOINT_FIELD.name))?;
        let transaction_index = value
            .transaction_index
            .ok_or_else(|| TryFromProtoError::missing(EventItem::TRANSACTION_INDEX_FIELD.name))?;
        let event_index = value
            .event_index
            .ok_or_else(|| TryFromProtoError::missing(EventItem::EVENT_INDEX_FIELD.name))?;
        let transaction_digest = value
            .transaction_digest
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing(EventItem::TRANSACTION_DIGEST_FIELD.name))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(EventItem::TRANSACTION_DIGEST_FIELD, e))?;
        let event_proto = value
            .event
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing(EventItem::EVENT_FIELD.name))?;
        let event =
            Event::try_from(event_proto).map_err(|e| e.nested(EventItem::EVENT_FIELD.name))?;

        Ok(Self {
            checkpoint,
            transaction_index,
            event_index,
            transaction_digest,
            event,
        })
    }
}

impl From<&AuthenticatedEvent> for EventItem {
    fn from(value: &AuthenticatedEvent) -> Self {
        Self {
            // `cursor` is server-assigned; leave unset on the way out.
            cursor: None,
            checkpoint: Some(value.checkpoint),
            event_index: Some(value.event_index),
            transaction_digest: Some(value.transaction_digest.to_string()),
            event: Some(value.event.clone().into()),
            transaction_index: Some(value.transaction_index),
        }
    }
}

impl From<AuthenticatedEvent> for EventItem {
    fn from(value: AuthenticatedEvent) -> Self {
        (&value).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proto::sui::rpc::v2::Bcs;
    use crate::proto::sui::rpc::v2::Event as ProtoEvent;
    use sui_sdk_types::Address;
    use sui_sdk_types::Identifier;
    use sui_sdk_types::StructTag;

    fn sample_event() -> Event {
        Event {
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
        }
    }

    fn sample_authenticated_event() -> AuthenticatedEvent {
        AuthenticatedEvent {
            checkpoint: 42,
            transaction_index: 3,
            event_index: 7,
            transaction_digest: Digest::new([0xaa; 32]),
            event: sample_event(),
        }
    }

    #[test]
    fn round_trip_through_proto_preserves_envelope() {
        let original = sample_authenticated_event();
        let proto: EventItem = (&original).into();
        let back = AuthenticatedEvent::try_from(&proto).unwrap();
        assert_eq!(back, original);
    }

    #[test]
    fn outbound_conversion_leaves_cursor_unset() {
        let proto: EventItem = (&sample_authenticated_event()).into();
        assert!(proto.cursor.is_none(), "cursor must be server-assigned");
    }

    #[test]
    fn missing_checkpoint_is_rejected() {
        let mut proto: EventItem = (&sample_authenticated_event()).into();
        proto.checkpoint = None;
        let err = AuthenticatedEvent::try_from(&proto).unwrap_err();
        assert_eq!(err.field_violation().field, "checkpoint");
    }

    #[test]
    fn missing_transaction_index_is_rejected() {
        let mut proto: EventItem = (&sample_authenticated_event()).into();
        proto.transaction_index = None;
        let err = AuthenticatedEvent::try_from(&proto).unwrap_err();
        assert_eq!(err.field_violation().field, "transaction_index");
    }

    #[test]
    fn missing_event_index_is_rejected() {
        let mut proto: EventItem = (&sample_authenticated_event()).into();
        proto.event_index = None;
        let err = AuthenticatedEvent::try_from(&proto).unwrap_err();
        assert_eq!(err.field_violation().field, "event_index");
    }

    #[test]
    fn missing_transaction_digest_is_rejected() {
        let mut proto: EventItem = (&sample_authenticated_event()).into();
        proto.transaction_digest = None;
        let err = AuthenticatedEvent::try_from(&proto).unwrap_err();
        assert_eq!(err.field_violation().field, "transaction_digest");
    }

    #[test]
    fn missing_event_is_rejected() {
        let mut proto: EventItem = (&sample_authenticated_event()).into();
        proto.event = None;
        let err = AuthenticatedEvent::try_from(&proto).unwrap_err();
        assert_eq!(err.field_violation().field, "event");
    }

    /// A malformed inner `Event` field surfaces with a field path that
    /// names the parent `event` field so the failure points at the
    /// containing slot in `EventItem`.
    #[test]
    fn malformed_inner_event_reports_nested_field_path() {
        let mut proto: EventItem = (&sample_authenticated_event()).into();
        // Strip the inner event's required `package_id`.
        proto.event = Some(ProtoEvent {
            package_id: None,
            module: Some("clock".into()),
            sender: Some(Address::TWO.to_string()),
            event_type: Some("0x2::clock::Tick".into()),
            contents: Some(Bcs::from(prost::bytes::Bytes::from_static(&[0x00]))),
            json: None,
        });
        let err = AuthenticatedEvent::try_from(&proto).unwrap_err();
        let path = &err.field_violation().field;
        assert!(
            path.starts_with("event."),
            "field path should be nested under the parent `event` field, got {path}",
        );
    }

    /// A malformed `transaction_digest` (base58-undecodable) reports the
    /// `transaction_digest` field with a parse-error source.
    #[test]
    fn malformed_transaction_digest_is_rejected() {
        let mut proto: EventItem = (&sample_authenticated_event()).into();
        proto.transaction_digest = Some("not-a-real-digest!".into());
        let err = AuthenticatedEvent::try_from(&proto).unwrap_err();
        assert_eq!(err.field_violation().field, "transaction_digest");
    }
}
