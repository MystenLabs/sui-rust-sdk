#![cfg_attr(doc_cfg, feature(doc_cfg))]

#[cfg(feature = "hash")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "hash")))]
pub mod hash;

mod address;
mod checkpoint;
mod crypto;
mod digest;
mod effects;
mod events;
mod execution_status;
pub mod framework;
mod gas;
mod object;
mod object_id;
mod transaction;
mod type_tag;
mod u256;

pub use address::Address;
pub use address::AddressParseError;
pub use checkpoint::CheckpointCommitment;
pub use checkpoint::CheckpointContents;
pub use checkpoint::CheckpointData;
pub use checkpoint::CheckpointSequenceNumber;
pub use checkpoint::CheckpointSummary;
pub use checkpoint::CheckpointTimestamp;
pub use checkpoint::CheckpointTransaction;
pub use checkpoint::CheckpointTransactionInfo;
pub use checkpoint::EndOfEpochData;
pub use checkpoint::EpochId;
pub use checkpoint::ProtocolVersion;
pub use checkpoint::SignedCheckpointSummary;
pub use checkpoint::StakeUnit;
pub use crypto::Bls12381PublicKey;
pub use crypto::Bls12381Signature;
pub use crypto::Bn254FieldElement;
pub use crypto::CircomG1;
pub use crypto::CircomG2;
pub use crypto::Claim;
pub use crypto::Ed25519PublicKey;
pub use crypto::Ed25519Signature;
pub use crypto::Intent;
pub use crypto::IntentAppId;
pub use crypto::IntentScope;
pub use crypto::IntentVersion;
pub use crypto::Jwk;
pub use crypto::JwkId;
pub use crypto::MultisigAggregatedSignature;
pub use crypto::MultisigCommittee;
pub use crypto::MultisigMember;
pub use crypto::MultisigMemberPublicKey;
pub use crypto::MultisigMemberSignature;
pub use crypto::PasskeyAuthenticator;
pub use crypto::PasskeyPublicKey;
pub use crypto::Secp256k1PublicKey;
pub use crypto::Secp256k1Signature;
pub use crypto::Secp256r1PublicKey;
pub use crypto::Secp256r1Signature;
pub use crypto::SignatureScheme;
pub use crypto::SimpleSignature;
pub use crypto::UserSignature;
pub use crypto::ValidatorAggregatedSignature;
pub use crypto::ValidatorCommittee;
pub use crypto::ValidatorCommitteeMember;
pub use crypto::ValidatorSignature;
pub use crypto::ZkLoginAuthenticator;
pub use crypto::ZkLoginInputs;
pub use crypto::ZkLoginProof;
pub use crypto::ZkLoginPublicIdentifier;
pub use digest::CheckpointContentsDigest;
pub use digest::CheckpointDigest;
pub use digest::ConsensusCommitDigest;
pub use digest::Digest;
pub use digest::DigestParseError;
pub use digest::EffectsAuxiliaryDataDigest;
pub use digest::ObjectDigest;
pub use digest::SigningDigest;
pub use digest::TransactionDigest;
pub use digest::TransactionEffectsDigest;
pub use digest::TransactionEventsDigest;
pub use effects::ChangedObject;
pub use effects::IdOperation;
pub use effects::ModifiedAtVersion;
pub use effects::ObjectIn;
pub use effects::ObjectOut;
pub use effects::ObjectReferenceWithOwner;
pub use effects::TransactionEffects;
pub use effects::TransactionEffectsV1;
pub use effects::TransactionEffectsV2;
pub use effects::UnchangedSharedKind;
pub use effects::UnchangedSharedObject;
pub use events::BalanceChange;
pub use events::Event;
pub use events::TransactionEvents;
pub use execution_status::CommandArgumentError;
pub use execution_status::ExecutionError;
pub use execution_status::ExecutionStatus;
pub use execution_status::MoveLocation;
pub use execution_status::PackageUpgradeError;
pub use execution_status::TypeArgumentError;
pub use gas::GasCostSummary;
pub use object::GenesisObject;
pub use object::MovePackage;
pub use object::MoveStruct;
pub use object::Object;
pub use object::ObjectData;
pub use object::ObjectReference;
pub use object::ObjectType;
pub use object::Owner;
pub use object::TypeOrigin;
pub use object::UpgradeInfo;
pub use object::Version;
pub use object_id::ObjectId;
pub use transaction::ActiveJwk;
pub use transaction::Argument;
pub use transaction::AuthenticatorStateExpire;
pub use transaction::AuthenticatorStateUpdate;
pub use transaction::CancelledTransaction;
pub use transaction::ChangeEpoch;
pub use transaction::Command;
pub use transaction::ConsensusCommitPrologue;
pub use transaction::ConsensusCommitPrologueV2;
pub use transaction::ConsensusCommitPrologueV3;
pub use transaction::ConsensusDeterminedVersionAssignments;
pub use transaction::EndOfEpochTransactionKind;
pub use transaction::GasPayment;
pub use transaction::GenesisTransaction;
pub use transaction::Input;
pub use transaction::MakeMoveVector;
pub use transaction::MergeCoins;
pub use transaction::MoveCall;
pub use transaction::ProgrammableTransaction;
pub use transaction::Publish;
pub use transaction::RandomnessStateUpdate;
pub use transaction::SignedTransaction;
pub use transaction::SplitCoins;
pub use transaction::SystemPackage;
pub use transaction::Transaction;
pub use transaction::TransactionExpiration;
pub use transaction::TransactionKind;
pub use transaction::TransferObjects;
pub use transaction::Upgrade;
pub use transaction::VersionAssignment;
pub use type_tag::Identifier;
pub use type_tag::StructTag;
pub use type_tag::TypeParseError;
pub use type_tag::TypeTag;

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
pub(crate) use transaction::SignedTransactionWithIntentMessage;

#[cfg(test)]
mod serialization_proptests;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PersonalMessage<'a>(pub std::borrow::Cow<'a, [u8]>);

#[cfg(feature = "serde")]
mod _serde {
    use base64ct::Base64;
    use base64ct::Encoding;
    use serde::Deserialize;
    use serde::Deserializer;
    use serde::Serialize;
    use serde::Serializer;
    use serde_with::Bytes;
    use serde_with::DeserializeAs;
    use serde_with::SerializeAs;
    use std::borrow::Cow;

    pub(crate) type ReadableDisplay =
        ::serde_with::As<::serde_with::IfIsHumanReadable<::serde_with::DisplayFromStr>>;

    pub(crate) type OptionReadableDisplay =
        ::serde_with::As<Option<::serde_with::IfIsHumanReadable<::serde_with::DisplayFromStr>>>;

    pub(crate) type ReadableBase64Encoded =
        ::serde_with::As<::serde_with::IfIsHumanReadable<Base64Encoded, ::serde_with::Bytes>>;

    pub(crate) struct Base64Encoded;

    impl<T: AsRef<[u8]>> SerializeAs<T> for Base64Encoded {
        fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let bytes = source.as_ref();
            let b64 = Base64::encode_string(bytes);
            b64.serialize(serializer)
        }
    }

    impl<'de, T: TryFrom<Vec<u8>>> DeserializeAs<'de, T> for Base64Encoded {
        fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
        where
            D: Deserializer<'de>,
        {
            let b64: Cow<'de, str> = Deserialize::deserialize(deserializer)?;
            let bytes = Base64::decode_vec(&b64).map_err(serde::de::Error::custom)?;
            let length = bytes.len();
            T::try_from(bytes).map_err(|_| {
                serde::de::Error::custom(format_args!(
                    "Can't convert a Byte Vector of length {length} to the output type."
                ))
            })
        }
    }

    /// Serializes a bitmap according to the roaring bitmap on-disk standard.
    /// <https://github.com/RoaringBitmap/RoaringFormatSpec>
    pub(crate) struct BinaryRoaringBitmap;

    impl SerializeAs<roaring::RoaringBitmap> for BinaryRoaringBitmap {
        fn serialize_as<S>(
            source: &roaring::RoaringBitmap,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut bytes = vec![];

            source
                .serialize_into(&mut bytes)
                .map_err(serde::ser::Error::custom)?;
            Bytes::serialize_as(&bytes, serializer)
        }
    }

    impl<'de> DeserializeAs<'de, roaring::RoaringBitmap> for BinaryRoaringBitmap {
        fn deserialize_as<D>(deserializer: D) -> Result<roaring::RoaringBitmap, D::Error>
        where
            D: Deserializer<'de>,
        {
            let bytes: Cow<'de, [u8]> = Bytes::deserialize_as(deserializer)?;
            roaring::RoaringBitmap::deserialize_from(&bytes[..]).map_err(serde::de::Error::custom)
        }
    }

    pub(crate) struct Base64RoaringBitmap;

    impl SerializeAs<roaring::RoaringBitmap> for Base64RoaringBitmap {
        fn serialize_as<S>(
            source: &roaring::RoaringBitmap,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut bytes = vec![];

            source
                .serialize_into(&mut bytes)
                .map_err(serde::ser::Error::custom)?;
            let b64 = Base64::encode_string(&bytes);
            b64.serialize(serializer)
        }
    }

    impl<'de> DeserializeAs<'de, roaring::RoaringBitmap> for Base64RoaringBitmap {
        fn deserialize_as<D>(deserializer: D) -> Result<roaring::RoaringBitmap, D::Error>
        where
            D: Deserializer<'de>,
        {
            let b64: Cow<'de, str> = Deserialize::deserialize(deserializer)?;
            let bytes = Base64::decode_vec(&b64).map_err(serde::de::Error::custom)?;
            roaring::RoaringBitmap::deserialize_from(&bytes[..]).map_err(serde::de::Error::custom)
        }
    }

    pub(crate) use super::SignedTransactionWithIntentMessage;
}
