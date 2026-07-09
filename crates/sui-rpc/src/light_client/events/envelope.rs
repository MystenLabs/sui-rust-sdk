//! SDK-side envelope for authenticated events received via the v2alpha
//! `LedgerService.ListEvents` RPC filtered by `EventStreamHeadFilter`.

use sui_sdk_types::Digest;
use sui_sdk_types::Event;

use crate::proto::TryFromProtoError;
use crate::proto::sui::rpc::v2::Event as ProtoEvent;
use crate::proto::sui::rpc::v2alpha::ListEventsResponse;

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

impl TryFrom<&ListEventsResponse> for AuthenticatedEvent {
    type Error = TryFromProtoError;

    fn try_from(value: &ListEventsResponse) -> Result<Self, Self::Error> {
        let event_proto = value
            .event
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing(ListEventsResponse::EVENT_FIELD.name))?;

        // The locating fields now live on the embedded `Event`; nest their
        // violations under the `event` slot so the reported path points at
        // the containing field.
        let missing = |field: &'static str| {
            TryFromProtoError::missing(field).nested(ListEventsResponse::EVENT_FIELD.name)
        };

        let checkpoint = event_proto
            .checkpoint
            .ok_or_else(|| missing(ProtoEvent::CHECKPOINT_FIELD.name))?;
        let transaction_index = event_proto
            .transaction_index
            .ok_or_else(|| missing(ProtoEvent::TRANSACTION_INDEX_FIELD.name))?;
        let event_index = event_proto
            .event_index
            .ok_or_else(|| missing(ProtoEvent::EVENT_INDEX_FIELD.name))?;
        let transaction_digest = event_proto
            .transaction_digest
            .as_ref()
            .ok_or_else(|| missing(ProtoEvent::TRANSACTION_DIGEST_FIELD.name))?
            .parse()
            .map_err(|e| {
                TryFromProtoError::invalid(ProtoEvent::TRANSACTION_DIGEST_FIELD, e)
                    .nested(ListEventsResponse::EVENT_FIELD.name)
            })?;
        let event = Event::try_from(event_proto)
            .map_err(|e| e.nested(ListEventsResponse::EVENT_FIELD.name))?;

        Ok(Self {
            checkpoint,
            transaction_index,
            event_index,
            transaction_digest,
            event,
        })
    }
}

impl From<&AuthenticatedEvent> for ListEventsResponse {
    fn from(value: &AuthenticatedEvent) -> Self {
        let event = ProtoEvent::from(value.event.clone())
            .with_checkpoint(value.checkpoint)
            .with_transaction_digest(value.transaction_digest.to_string())
            .with_transaction_index(value.transaction_index)
            .with_event_index(value.event_index);
        Self {
            event: Some(event),
            // `watermark` and `end` are server-assigned; leave unset on
            // the way out.
            watermark: None,
            end: None,
        }
    }
}

impl From<AuthenticatedEvent> for ListEventsResponse {
    fn from(value: AuthenticatedEvent) -> Self {
        (&value).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let proto: ListEventsResponse = (&original).into();
        let back = AuthenticatedEvent::try_from(&proto).unwrap();
        assert_eq!(back, original);
    }

    #[test]
    fn outbound_conversion_leaves_server_fields_unset() {
        let proto: ListEventsResponse = (&sample_authenticated_event()).into();
        assert!(
            proto.watermark.is_none(),
            "watermark (cursor + checkpoint) must be server-assigned"
        );
        assert!(proto.end.is_none(), "end must be server-assigned");
    }

    #[test]
    fn missing_checkpoint_is_rejected() {
        let mut proto: ListEventsResponse = (&sample_authenticated_event()).into();
        proto.event.as_mut().unwrap().checkpoint = None;
        let err = AuthenticatedEvent::try_from(&proto).unwrap_err();
        assert_eq!(err.field_violation().field, "event.checkpoint");
    }

    #[test]
    fn missing_transaction_index_is_rejected() {
        let mut proto: ListEventsResponse = (&sample_authenticated_event()).into();
        proto.event.as_mut().unwrap().transaction_index = None;
        let err = AuthenticatedEvent::try_from(&proto).unwrap_err();
        assert_eq!(err.field_violation().field, "event.transaction_index");
    }

    #[test]
    fn missing_event_index_is_rejected() {
        let mut proto: ListEventsResponse = (&sample_authenticated_event()).into();
        proto.event.as_mut().unwrap().event_index = None;
        let err = AuthenticatedEvent::try_from(&proto).unwrap_err();
        assert_eq!(err.field_violation().field, "event.event_index");
    }

    #[test]
    fn missing_transaction_digest_is_rejected() {
        let mut proto: ListEventsResponse = (&sample_authenticated_event()).into();
        proto.event.as_mut().unwrap().transaction_digest = None;
        let err = AuthenticatedEvent::try_from(&proto).unwrap_err();
        assert_eq!(err.field_violation().field, "event.transaction_digest");
    }

    #[test]
    fn missing_event_is_rejected() {
        let mut proto: ListEventsResponse = (&sample_authenticated_event()).into();
        proto.event = None;
        let err = AuthenticatedEvent::try_from(&proto).unwrap_err();
        assert_eq!(err.field_violation().field, "event");
    }

    /// A malformed inner `Event` field surfaces with a field path that
    /// names the parent `event` field so the failure points at the
    /// containing slot in `ListEventsResponse`.
    #[test]
    fn malformed_inner_event_reports_nested_field_path() {
        let mut proto: ListEventsResponse = (&sample_authenticated_event()).into();
        // Strip the inner event's required `package_id`.
        proto.event.as_mut().unwrap().package_id = None;
        let err = AuthenticatedEvent::try_from(&proto).unwrap_err();
        let path = &err.field_violation().field;
        assert!(
            path.starts_with("event."),
            "field path should be nested under the parent `event` field, got {path}",
        );
    }

    /// A malformed `transaction_digest` (base58-undecodable) reports the
    /// `event.transaction_digest` field with a parse-error source.
    #[test]
    fn malformed_transaction_digest_is_rejected() {
        let mut proto: ListEventsResponse = (&sample_authenticated_event()).into();
        proto.event.as_mut().unwrap().transaction_digest = Some("not-a-real-digest!".into());
        let err = AuthenticatedEvent::try_from(&proto).unwrap_err();
        assert_eq!(err.field_violation().field, "event.transaction_digest");
    }
}
