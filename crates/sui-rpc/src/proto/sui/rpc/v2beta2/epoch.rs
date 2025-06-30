use super::*;
use crate::field::FieldMaskTree;
use crate::merge::Merge;

impl Merge<&Epoch> for Epoch {
    fn merge(&mut self, source: &Epoch, mask: &FieldMaskTree) {
        let Epoch {
            epoch,
            committee,
            system_state,
            first_checkpoint,
            last_checkpoint,
            start,
            end,
            reference_gas_price,
            protocol_config,
        } = source;

        if mask.contains(Self::EPOCH_FIELD.name) {
            self.epoch = *epoch;
        }

        if mask.contains(Self::COMMITTEE_FIELD.name) {
            self.committee = committee.to_owned();
        }

        if mask.contains(Self::SYSTEM_STATE_FIELD.name) {
            self.system_state = system_state.to_owned();
        }

        if mask.contains(Self::FIRST_CHECKPOINT_FIELD.name) {
            self.first_checkpoint = first_checkpoint.to_owned();
        }

        if mask.contains(Self::LAST_CHECKPOINT_FIELD.name) {
            self.last_checkpoint = last_checkpoint.to_owned();
        }

        if mask.contains(Self::START_FIELD.name) {
            self.start = start.to_owned();
        }

        if mask.contains(Self::END_FIELD.name) {
            self.end = end.to_owned();
        }

        if mask.contains(Self::REFERENCE_GAS_PRICE_FIELD.name) {
            self.reference_gas_price = reference_gas_price.to_owned();
        }

        if let Some(submask) = mask.subtree(Self::PROTOCOL_CONFIG_FIELD.name) {
            self.protocol_config = protocol_config
                .as_ref()
                .map(|config| ProtocolConfig::merge_from(config, &submask));
        }
    }
}

impl Merge<&ProtocolConfig> for ProtocolConfig {
    fn merge(&mut self, source: &ProtocolConfig, mask: &FieldMaskTree) {
        let ProtocolConfig {
            protocol_version,
            feature_flags,
            attributes,
        } = source;

        if mask.contains(Self::PROTOCOL_VERSION_FIELD.name) {
            self.protocol_version = *protocol_version;
        }

        if mask.contains(Self::FEATURE_FLAGS_FIELD.name) {
            self.feature_flags = feature_flags.to_owned();
        }

        if mask.contains(Self::ATTRIBUTES_FIELD.name) {
            self.attributes = attributes.to_owned();
        }
    }
}

impl Merge<ProtocolConfig> for ProtocolConfig {
    fn merge(&mut self, source: ProtocolConfig, mask: &FieldMaskTree) {
        let ProtocolConfig {
            protocol_version,
            feature_flags,
            attributes,
        } = source;

        if mask.contains(Self::PROTOCOL_VERSION_FIELD.name) {
            self.protocol_version = protocol_version;
        }

        if mask.contains(Self::FEATURE_FLAGS_FIELD.name) {
            self.feature_flags = feature_flags;
        }

        if mask.contains(Self::ATTRIBUTES_FIELD.name) {
            self.attributes = attributes;
        }
    }
}
