use super::*;
use crate::field::FieldMaskTree;
use crate::merge::Merge;
use crate::proto::TryFromProtoError;

//
// Event
//

impl From<sui_sdk_types::Event> for Event {
    fn from(value: sui_sdk_types::Event) -> Self {
        Self::merge_from(value, &FieldMaskTree::new_wildcard())
    }
}

impl Merge<sui_sdk_types::Event> for Event {
    fn merge(&mut self, source: sui_sdk_types::Event, mask: &FieldMaskTree) {
        if mask.contains(Self::PACKAGE_ID_FIELD.name) {
            self.package_id = Some(source.package_id.to_string());
        }

        if mask.contains(Self::MODULE_FIELD.name) {
            self.module = Some(source.module.to_string());
        }

        if mask.contains(Self::SENDER_FIELD.name) {
            self.sender = Some(source.sender.to_string());
        }

        if mask.contains(Self::EVENT_TYPE_FIELD.name) {
            self.event_type = Some(source.type_.to_string());
        }

        if mask.contains(Self::CONTENTS_FIELD.name) {
            self.contents = Some(Bcs {
                name: Some(source.type_.to_string()),
                value: Some(source.contents.into()),
            });
        }
    }
}

impl Merge<&Event> for Event {
    fn merge(&mut self, source: &Event, mask: &FieldMaskTree) {
        let Event {
            package_id,
            module,
            sender,
            event_type,
            contents,
            json,
        } = source;

        if mask.contains(Self::PACKAGE_ID_FIELD.name) {
            self.package_id = package_id.clone();
        }

        if mask.contains(Self::MODULE_FIELD.name) {
            self.module = module.clone();
        }

        if mask.contains(Self::SENDER_FIELD.name) {
            self.sender = sender.clone();
        }

        if mask.contains(Self::EVENT_TYPE_FIELD.name) {
            self.event_type = event_type.clone();
        }

        if mask.contains(Self::CONTENTS_FIELD.name) {
            self.contents = contents.clone();
        }

        if mask.contains(Self::JSON_FIELD.name) {
            self.json = json.clone();
        }
    }
}

impl TryFrom<&Event> for sui_sdk_types::Event {
    type Error = TryFromProtoError;

    fn try_from(value: &Event) -> Result<Self, Self::Error> {
        let package_id = value
            .package_id
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("package_id"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(Event::PACKAGE_ID_FIELD, e))?;

        let module = value
            .module
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("module"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(Event::MODULE_FIELD, e))?;

        let sender = value
            .sender
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("sender"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(Event::SENDER_FIELD, e))?;

        let type_ = value
            .event_type
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("event_type"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(Event::EVENT_TYPE_FIELD, e))?;

        let contents = value
            .contents
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("contents"))?
            .value()
            .to_vec();

        Ok(Self {
            package_id,
            module,
            sender,
            type_,
            contents,
        })
    }
}

//
// TransactionEvents
//

impl From<sui_sdk_types::TransactionEvents> for TransactionEvents {
    fn from(value: sui_sdk_types::TransactionEvents) -> Self {
        Self::merge_from(value, &FieldMaskTree::new_wildcard())
    }
}

impl Merge<sui_sdk_types::TransactionEvents> for TransactionEvents {
    fn merge(&mut self, source: sui_sdk_types::TransactionEvents, mask: &FieldMaskTree) {
        if mask.contains(Self::BCS_FIELD.name) {
            let mut bcs = super::Bcs::serialize(&source).unwrap();
            bcs.name = Some("TransactionEvents".to_owned());
            self.bcs = Some(bcs);
        }

        if mask.contains(Self::DIGEST_FIELD.name) {
            self.digest = Some(source.digest().to_string());
        }

        if let Some(events_mask) = mask.subtree(Self::EVENTS_FIELD.name) {
            self.events = source
                .0
                .into_iter()
                .map(|event| Event::merge_from(event, &events_mask))
                .collect();
        }
    }
}

impl Merge<&TransactionEvents> for TransactionEvents {
    fn merge(&mut self, source: &TransactionEvents, mask: &FieldMaskTree) {
        let TransactionEvents {
            bcs,
            digest,
            events,
        } = source;

        if mask.contains(Self::BCS_FIELD.name) {
            self.bcs = bcs.clone();
        }

        if mask.contains(Self::DIGEST_FIELD.name) {
            self.digest = digest.clone();
        }

        if let Some(events_mask) = mask.subtree(Self::EVENTS_FIELD.name) {
            self.events = events
                .iter()
                .map(|event| Event::merge_from(event, &events_mask))
                .collect();
        }
    }
}

impl TryFrom<&TransactionEvents> for sui_sdk_types::TransactionEvents {
    type Error = TryFromProtoError;

    fn try_from(value: &TransactionEvents) -> Result<Self, Self::Error> {
        Ok(Self(
            value
                .events
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
        ))
    }
}
