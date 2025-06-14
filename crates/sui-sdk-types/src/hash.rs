use crate::Address;
use crate::Digest;

use blake2::Digest as DigestTrait;

type Blake2b256 = blake2::Blake2b<blake2::digest::consts::U32>;

/// A Blake2b256 Hasher
#[derive(Debug, Default)]
pub struct Hasher(Blake2b256);

impl Hasher {
    /// Initialize a new Blake2b256 Hasher instance.
    pub fn new() -> Self {
        Self(Blake2b256::new())
    }

    /// Process the provided data, updating internal state.
    pub fn update<T: AsRef<[u8]>>(&mut self, data: T) {
        self.0.update(data)
    }

    /// Finalize hashing, consuming the Hasher instance and returning the resultant hash or
    /// `Digest`.
    pub fn finalize(self) -> Digest {
        let mut buf = [0; Digest::LENGTH];
        let result = self.0.finalize();

        buf.copy_from_slice(result.as_slice());

        Digest::new(buf)
    }

    /// Convenience function for creating a new Hasher instance, hashing the provided data, and
    /// returning the resultant `Digest`
    pub fn digest<T: AsRef<[u8]>>(data: T) -> Digest {
        let mut hasher = Self::new();
        hasher.update(data);
        hasher.finalize()
    }
}

impl std::io::Write for Hasher {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}

impl crate::Ed25519PublicKey {
    /// Derive an `Address` from this Public Key
    ///
    /// An `Address` can be derived from an `Ed25519PublicKey` by hashing the bytes of the public
    /// key prefixed with the Ed25519 `SignatureScheme` flag (`0x00`).
    ///
    /// `hash( 0x00 || 32-byte ed25519 public key)`
    ///
    /// ```
    /// use sui_sdk_types::hash::Hasher;
    /// use sui_sdk_types::Address;
    /// use sui_sdk_types::Ed25519PublicKey;
    ///
    /// let public_key_bytes = [0; 32];
    /// let mut hasher = Hasher::new();
    /// hasher.update([0x00]); // The SignatureScheme flag for Ed25519 is `0`
    /// hasher.update(public_key_bytes);
    /// let address = Address::new(hasher.finalize().into_inner());
    /// println!("Address: {}", address);
    ///
    /// let public_key = Ed25519PublicKey::new(public_key_bytes);
    /// assert_eq!(address, public_key.derive_address());
    /// ```
    pub fn derive_address(&self) -> Address {
        let mut hasher = Hasher::new();
        self.write_into_hasher(&mut hasher);
        let digest = hasher.finalize();
        Address::new(digest.into_inner())
    }

    fn write_into_hasher(&self, hasher: &mut Hasher) {
        hasher.update([self.scheme().to_u8()]);
        hasher.update(self.inner());
    }
}

impl crate::Secp256k1PublicKey {
    /// Derive an `Address` from this Public Key
    ///
    /// An `Address` can be derived from a `Secp256k1PublicKey` by hashing the bytes of the public
    /// key prefixed with the Secp256k1 `SignatureScheme` flag (`0x01`).
    ///
    /// `hash( 0x01 || 33-byte secp256k1 public key)`
    ///
    /// ```
    /// use sui_sdk_types::hash::Hasher;
    /// use sui_sdk_types::Address;
    /// use sui_sdk_types::Secp256k1PublicKey;
    ///
    /// let public_key_bytes = [0; 33];
    /// let mut hasher = Hasher::new();
    /// hasher.update([0x01]); // The SignatureScheme flag for Secp256k1 is `1`
    /// hasher.update(public_key_bytes);
    /// let address = Address::new(hasher.finalize().into_inner());
    /// println!("Address: {}", address);
    ///
    /// let public_key = Secp256k1PublicKey::new(public_key_bytes);
    /// assert_eq!(address, public_key.derive_address());
    /// ```
    pub fn derive_address(&self) -> Address {
        let mut hasher = Hasher::new();
        self.write_into_hasher(&mut hasher);
        let digest = hasher.finalize();
        Address::new(digest.into_inner())
    }

    fn write_into_hasher(&self, hasher: &mut Hasher) {
        hasher.update([self.scheme().to_u8()]);
        hasher.update(self.inner());
    }
}

impl crate::Secp256r1PublicKey {
    /// Derive an `Address` from this Public Key
    ///
    /// An `Address` can be derived from a `Secp256r1PublicKey` by hashing the bytes of the public
    /// key prefixed with the Secp256r1 `SignatureScheme` flag (`0x02`).
    ///
    /// `hash( 0x02 || 33-byte secp256r1 public key)`
    ///
    /// ```
    /// use sui_sdk_types::hash::Hasher;
    /// use sui_sdk_types::Address;
    /// use sui_sdk_types::Secp256r1PublicKey;
    ///
    /// let public_key_bytes = [0; 33];
    /// let mut hasher = Hasher::new();
    /// hasher.update([0x02]); // The SignatureScheme flag for Secp256r1 is `2`
    /// hasher.update(public_key_bytes);
    /// let address = Address::new(hasher.finalize().into_inner());
    /// println!("Address: {}", address);
    ///
    /// let public_key = Secp256r1PublicKey::new(public_key_bytes);
    /// assert_eq!(address, public_key.derive_address());
    /// ```
    pub fn derive_address(&self) -> Address {
        let mut hasher = Hasher::new();
        self.write_into_hasher(&mut hasher);
        let digest = hasher.finalize();
        Address::new(digest.into_inner())
    }

    fn write_into_hasher(&self, hasher: &mut Hasher) {
        hasher.update([self.scheme().to_u8()]);
        hasher.update(self.inner());
    }
}

impl crate::ZkLoginPublicIdentifier {
    /// Derive an `Address` from this `ZkLoginPublicIdentifier` by hashing the byte length of the
    /// `iss` followed by the `iss` bytes themselves and the full 32 byte `address_seed` value, all
    /// prefixed with the zklogin `SignatureScheme` flag (`0x05`).
    ///
    /// `hash( 0x05 || iss_bytes_len || iss_bytes || 32_byte_address_seed )`
    pub fn derive_address_padded(&self) -> Address {
        let mut hasher = Hasher::new();
        self.write_into_hasher_padded(&mut hasher);
        let digest = hasher.finalize();
        Address::new(digest.into_inner())
    }

    fn write_into_hasher_padded(&self, hasher: &mut Hasher) {
        hasher.update([self.scheme().to_u8()]);
        hasher.update([self.iss().len() as u8]); // TODO enforce iss is less than 255 bytes
        hasher.update(self.iss());
        hasher.update(self.address_seed().padded());
    }

    /// Derive an `Address` from this `ZkLoginPublicIdentifier` by hashing the byte length of the
    /// `iss` followed by the `iss` bytes themselves and the `address_seed` bytes with any leading
    /// zero-bytes stripped, all prefixed with the zklogin `SignatureScheme` flag (`0x05`).
    ///
    /// `hash( 0x05 || iss_bytes_len || iss_bytes || unpadded_32_byte_address_seed )`
    pub fn derive_address_unpadded(&self) -> Address {
        let mut hasher = Hasher::new();
        hasher.update([self.scheme().to_u8()]);
        hasher.update([self.iss().len() as u8]); // TODO enforce iss is less than 255 bytes
        hasher.update(self.iss());
        hasher.update(self.address_seed().unpadded());
        let digest = hasher.finalize();
        Address::new(digest.into_inner())
    }

    /// Provides an iterator over the addresses that correspond to this zklogin authenticator.
    ///
    /// In the majority of instances this will only yield a single address, except for the
    /// instances where the `address_seed` value has a leading zero-byte, in such cases the
    /// returned iterator will yield two addresses.
    pub fn derive_address(&self) -> impl Iterator<Item = Address> {
        let main_address = self.derive_address_padded();
        let mut addresses = [Some(main_address), None];
        // If address_seed starts with a zero byte then we know that this zklogin authenticator has
        // two addresses
        if self.address_seed().padded()[0] == 0 {
            let secondary_address = self.derive_address_unpadded();

            addresses[1] = Some(secondary_address);
        }

        addresses.into_iter().flatten()
    }
}

impl crate::PasskeyPublicKey {
    /// Derive an `Address` from this Passkey Public Key
    ///
    /// An `Address` can be derived from a `PasskeyPublicKey` by hashing the bytes of the
    /// `Secp256r1PublicKey` that corresponds to this passkey prefixed with the Passkey
    /// `SignatureScheme` flag (`0x06`).
    ///
    /// `hash( 0x06 || 33-byte secp256r1-public-key)`
    pub fn derive_address(&self) -> Address {
        let mut hasher = Hasher::new();
        self.write_into_hasher(&mut hasher);
        let digest = hasher.finalize();
        Address::new(digest.into_inner())
    }

    fn write_into_hasher(&self, hasher: &mut Hasher) {
        hasher.update([self.scheme().to_u8()]);
        hasher.update(self.inner().inner());
    }
}

impl crate::MultisigCommittee {
    /// Derive an `Address` from this MultisigCommittee.
    ///
    /// A MultiSig address
    /// is defined as the 32-byte Blake2b hash of serializing the `SignatureScheme` flag (0x03), the
    /// threshold (in little endian), and the concatenation of all n flag, public keys and
    /// its weight.
    ///
    /// `hash(0x03 || threshold || flag_1 || pk_1 || weight_1
    /// || ... || flag_n || pk_n || weight_n)`.
    ///
    /// When flag_i is ZkLogin, the pk_i for the [`ZkLoginPublicIdentifier`] refers to the same
    /// input used when deriving the address using the
    /// [`ZkLoginPublicIdentifier::derive_address_padded`] method (using the full 32-byte
    /// `address_seed` value).
    ///
    /// [`ZkLoginPublicIdentifier`]: crate::ZkLoginPublicIdentifier
    /// [`ZkLoginPublicIdentifier::derive_address_padded`]: crate::ZkLoginPublicIdentifier::derive_address_padded
    pub fn derive_address(&self) -> Address {
        use crate::MultisigMemberPublicKey::*;

        let mut hasher = Hasher::new();
        hasher.update([self.scheme().to_u8()]);
        hasher.update(self.threshold().to_le_bytes());

        for member in self.members() {
            match member.public_key() {
                Ed25519(p) => p.write_into_hasher(&mut hasher),
                Secp256k1(p) => p.write_into_hasher(&mut hasher),
                Secp256r1(p) => p.write_into_hasher(&mut hasher),
                ZkLogin(p) => p.write_into_hasher_padded(&mut hasher),
                Passkey(p) => p.write_into_hasher(&mut hasher),
            }

            hasher.update(member.weight().to_le_bytes());
        }

        let digest = hasher.finalize();
        Address::new(digest.into_inner())
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
mod type_digest {
    use super::Hasher;
    use crate::CheckpointContents;
    use crate::CheckpointContentsDigest;
    use crate::CheckpointDigest;
    use crate::CheckpointSummary;
    use crate::Digest;
    use crate::Object;
    use crate::ObjectDigest;
    use crate::Transaction;
    use crate::TransactionDigest;
    use crate::TransactionEffects;
    use crate::TransactionEffectsDigest;
    use crate::TransactionEvents;
    use crate::TransactionEventsDigest;

    impl Object {
        /// Calculate the digest of this `Object`
        ///
        /// This is done by hashing the BCS bytes of this `Object` prefixed
        pub fn digest(&self) -> ObjectDigest {
            const SALT: &str = "Object::";
            let digest = type_digest(SALT, self);
            ObjectDigest::new(digest.into_inner())
        }
    }

    impl CheckpointSummary {
        pub fn digest(&self) -> CheckpointDigest {
            const SALT: &str = "CheckpointSummary::";
            let digest = type_digest(SALT, self);
            CheckpointDigest::new(digest.into_inner())
        }
    }

    impl CheckpointContents {
        pub fn digest(&self) -> CheckpointContentsDigest {
            const SALT: &str = "CheckpointContents::";
            let digest = type_digest(SALT, self);
            CheckpointContentsDigest::new(digest.into_inner())
        }
    }

    impl Transaction {
        pub fn digest(&self) -> TransactionDigest {
            const SALT: &str = "TransactionData::";
            let digest = type_digest(SALT, self);
            TransactionDigest::new(digest.into_inner())
        }
    }

    impl TransactionEffects {
        pub fn digest(&self) -> TransactionEffectsDigest {
            const SALT: &str = "TransactionEffects::";
            let digest = type_digest(SALT, self);
            TransactionEffectsDigest::new(digest.into_inner())
        }
    }

    impl TransactionEvents {
        pub fn digest(&self) -> TransactionEventsDigest {
            const SALT: &str = "TransactionEvents::";
            let digest = type_digest(SALT, self);
            TransactionEventsDigest::new(digest.into_inner())
        }
    }

    fn type_digest<T: serde::Serialize>(salt: &str, ty: &T) -> Digest {
        let mut hasher = Hasher::new();
        hasher.update(salt);
        bcs::serialize_into(&mut hasher, ty).unwrap();
        hasher.finalize()
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
mod signing_message {
    use crate::hash::Hasher;
    use crate::Digest;
    use crate::Intent;
    use crate::IntentAppId;
    use crate::IntentScope;
    use crate::IntentVersion;
    use crate::PersonalMessage;
    use crate::SigningDigest;
    use crate::Transaction;

    impl Transaction {
        pub fn signing_digest(&self) -> SigningDigest {
            const INTENT: Intent = Intent {
                scope: IntentScope::TransactionData,
                version: IntentVersion::V0,
                app_id: IntentAppId::Sui,
            };
            let digest = signing_digest(INTENT, self);
            digest.into_inner()
        }
    }

    fn signing_digest<T: serde::Serialize + ?Sized>(intent: Intent, ty: &T) -> Digest {
        let mut hasher = Hasher::new();
        hasher.update(intent.to_bytes());
        bcs::serialize_into(&mut hasher, ty).unwrap();
        hasher.finalize()
    }

    impl PersonalMessage<'_> {
        pub fn signing_digest(&self) -> SigningDigest {
            const INTENT: Intent = Intent {
                scope: IntentScope::PersonalMessage,
                version: IntentVersion::V0,
                app_id: IntentAppId::Sui,
            };
            let digest = signing_digest(INTENT, &self.0);
            digest.into_inner()
        }
    }
}

/// A 1-byte domain separator for deriving `ObjectId`s in Sui. It is starting from `0xf0` to ensure
/// no hashing collision for any ObjectId vs Address which is derived as the hash of `flag ||
/// pubkey`.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
#[repr(u8)]
enum HashingIntent {
    #[cfg(feature = "serde")]
    ChildObjectId = 0xf0,
    RegularObjectId = 0xf1,
}

impl crate::ObjectId {
    /// Create an ObjectId from `TransactionDigest` and `count`.
    ///
    /// `count` is the number of objects that have been created during a transactions.
    pub fn derive_id(digest: crate::TransactionDigest, count: u64) -> Self {
        let mut hasher = Hasher::new();
        hasher.update([HashingIntent::RegularObjectId as u8]);
        hasher.update(digest);
        hasher.update(count.to_le_bytes());
        let digest = hasher.finalize();
        Self::new(digest.into_inner())
    }

    /// Derive an ObjectId for a Dynamic Child Object.
    ///
    /// hash(parent || len(key) || key || key_type_tag)
    #[cfg(feature = "serde")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
    pub fn derive_dynamic_child_id(&self, key_type_tag: &crate::TypeTag, key_bytes: &[u8]) -> Self {
        let mut hasher = Hasher::new();
        hasher.update([HashingIntent::ChildObjectId as u8]);
        hasher.update(self);
        hasher.update(
            u64::try_from(key_bytes.len())
                .expect("key_bytes must fit into a u64")
                .to_le_bytes(),
        );
        hasher.update(key_bytes);
        bcs::serialize_into(&mut hasher, key_type_tag)
            .expect("bcs serialization of `TypeTag` cannot fail");
        let digest = hasher.finalize();

        Self::new(digest.into_inner())
    }
}

#[cfg(test)]
mod test {
    use super::HashingIntent;
    use crate::SignatureScheme;
    use test_strategy::proptest;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    impl HashingIntent {
        fn from_byte(byte: u8) -> Result<Self, u8> {
            match byte {
                0xf0 => Ok(Self::ChildObjectId),
                0xf1 => Ok(Self::RegularObjectId),
                invalid => Err(invalid),
            }
        }
    }

    #[proptest]
    fn hashing_intent_does_not_overlap_with_signature_scheme(intent: HashingIntent) {
        SignatureScheme::from_byte(intent as u8).unwrap_err();
    }

    #[proptest]
    fn signature_scheme_does_not_overlap_with_hashing_intent(scheme: SignatureScheme) {
        HashingIntent::from_byte(scheme.to_u8()).unwrap_err();
    }

    #[proptest]
    fn roundtrip_hashing_intent(intent: HashingIntent) {
        assert_eq!(Ok(intent), HashingIntent::from_byte(intent as u8));
    }
}
