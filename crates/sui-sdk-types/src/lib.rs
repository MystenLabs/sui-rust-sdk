//! Core type definitions for the [Sui] blockchain.
//!
//! [Sui] is a next-generation smart contract platform with high throughput, low latency, and an
//! asset-oriented programming model powered by the Move programming language. This crate provides
//! type definitions for working with the data that makes up the [Sui] blockchain.
//!
//! [Sui]: https://sui.io
//!
//! # Feature flags
//!
//! This library uses a set of [feature flags] to reduce the number of dependencies and amount of
//! compiled code. By default, no features are enabled which allows one to enable a subset
//! specifically for their use case. Below is a list of the available feature flags.
//!
//! - `serde`: Enables support for serializing and deserializing types to/from BCS utilizing
//!            [serde] library.
//! - `rand`: Enables support for generating random instances of a number of types via the [rand]
//!           library.
//! - `hash`: Enables support for hashing, which is required for deriving addresses and calculating
//!           digests for various types.
//! - `proptest`: Enables support for the [proptest] library by providing implementations of
//!               [proptest::arbitrary::Arbitrary] for many types.
//!
//! [feature flags]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section
//! [serde]: https://docs.rs/serde
//! [rand]: https://docs.rs/rand
//! [proptest]: https://docs.rs/proptest
//! [proptest::arbitrary::Arbitrary]: https://docs.rs/proptest/latest/proptest/arbitrary/trait.Arbitrary.html
//!
//! # BCS
//!
//! [BCS] is the serialization format used to represent the state of the blockchain and is used
//! extensively throughout the Sui ecosystem. In particular the BCS format is leveraged because it
//! _"guarantees canonical serialization, meaning that for any given data type, there is a
//! one-to-one correspondence between in-memory values and valid byte representations."_ One
//! benefit of this property of having a canonical serialized representation is to allow different
//! entities in the ecosystem to all agree on how a particular type should be interpreted and more
//! importantly define a deterministic representation for hashing and signing.
//!
//! This library strives to guarantee that the types defined are fully BCS-compatible with the data
//! that the network produces. The one caveat to this would be that as the Sui protocol evolves,
//! new type variants are added and older versions of this library may not support those newly
//! added variants. The expectation is that the most recent release of this library will support
//! new variants and types as they are released to Sui's `testnet` network.
//!
//! See the documentation for the various types defined by this crate for a specification of their
//! BCS serialized representation which will be defined using ABNF notation as described by
//! [RFC-5234]. In addition to the format itself, some types have an extra layer of verification
//! and may impose additional restrictions on valid byte representations above and beyond those
//! already provided by BCS. In these instances the documentation for those types will clearly
//! specify these additional restrictions.
//!
//! Here are some common rules:
//!
//! ```text
//! ; --- BCS Value ---
//! bcs-value           = bcs-struct / bcs-enum / bcs-length-prefixed / bcs-fixed-length
//! bcs-length-prefixed = bytes / string / vector / option
//! bcs-fixed-length    = u8 / u16 / u32 / u64 / u128 /
//!                       i8 / i16 / i32 / i64 / i128 /
//!                       boolean
//! bcs-struct          = *bcs-value                ; Sequence of serialized fields
//! bcs-enum            = uleb128-index bcs-value   ; Enum index and associated value
//!
//! ; --- Length-prefixed types ---
//! bytes           = uleb128 *OCTET          ; Raw bytes of the specified length
//! string          = uleb128 *OCTET          ; valid utf8 string of the specified length
//! vector          = uleb128 *bcs-value      ; Length-prefixed list of values
//! option          = %x00 / (%x01 bcs-value) ; optional value
//!
//! ; --- Fixed-length types ---
//! u8          = OCTET                     ; 1-byte unsigned integer
//! u16         = 2OCTET                    ; 2-byte unsigned integer, little-endian
//! u32         = 4OCTET                    ; 4-byte unsigned integer, little-endian
//! u64         = 8OCTET                    ; 8-byte unsigned integer, little-endian
//! u128        = 16OCTET                   ; 16-byte unsigned integer, little-endian
//! i8          = OCTET                     ; 1-byte signed integer
//! i16         = 2OCTET                    ; 2-byte signed integer, little-endian
//! i32         = 4OCTET                    ; 4-byte signed integer, little-endian
//! i64         = 8OCTET                    ; 8-byte signed integer, little-endian
//! i128        = 16OCTET                   ; 16-byte signed integer, little-endian
//! boolean     = %x00 / %x01               ; Boolean: 0 = false, 1 = true
//! array       = *(bcs-value)              ; Fixed-length array
//!
//! ; --- ULEB128 definition ---
//! uleb128         = 1*5uleb128-byte       ; Variable-length ULEB128 encoding
//! uleb128-byte    = %x00-7F / %x80-FF     ; ULEB128 continuation rules
//! uleb128-index   = uleb128               ; ULEB128-encoded variant index
//! ```
//!
//! [BCS]: https://docs.rs/bcs
//! [RFC-5234]: https://datatracker.ietf.org/doc/html/rfc5234

#![cfg_attr(doc_cfg, feature(doc_cfg))]
// TODO finish documenting all public items
// #![warn(missing_docs)]

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

#[cfg(feature = "hash")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "hash")))]
pub mod hash;

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
pub use crypto::ZkLoginClaim;
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
pub use transaction::ConsensusCommitPrologueV4;
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
