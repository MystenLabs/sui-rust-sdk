use super::SimpleSignature;
use crate::checkpoint::EpochId;
use crate::u256::U256;

/// A zklogin authenticator
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// zklogin-bcs = bytes             ; contents are defined by <zklogin-authenticator>
/// zklogin     = zklogin-flag
///               zklogin-inputs
///               u64               ; max epoch
///               simple-signature    
/// ```
///
/// Note: Due to historical reasons, signatures are serialized slightly different from the majority
/// of the types in Sui. In particular if a signature is ever embedded in another structure it
/// generally is serialized as `bytes` meaning it has a length prefix that defines the length of
/// the completely serialized signature.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct ZkLoginAuthenticator {
    /// Zklogin proof and inputs required to perform proof verification.
    pub inputs: ZkLoginInputs,

    /// Maximum epoch for which the proof is valid.
    pub max_epoch: EpochId,

    /// User signature with the pubkey attested to by the provided proof.
    pub signature: SimpleSignature,
}

/// A zklogin groth16 proof and the required inputs to perform proof verification.
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// zklogin-inputs = zklogin-proof
///                  zklogin-claim
///                  string              ; base64url-unpadded encoded JwtHeader
///                  bn254-field-element ; address_seed
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct ZkLoginInputs {
    pub proof_points: ZkLoginProof,
    pub iss_base64_details: ZkLoginClaim,
    pub header_base64: String,
    pub address_seed: Bn254FieldElement,
}

impl ZkLoginInputs {
    #[cfg(feature = "serde")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
    pub fn iss(&self) -> Result<String, InvalidZkLoginClaimError> {
        const ISS: &str = "iss";

        let iss = self.iss_base64_details.verify_extended_claim(ISS)?;

        if iss.len() > 255 {
            Err(InvalidZkLoginClaimError::new("invalid iss: too long"))
        } else {
            Ok(iss)
        }
    }

    #[cfg(feature = "serde")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
    pub fn public_identifier(&self) -> Result<ZkLoginPublicIdentifier, InvalidZkLoginClaimError> {
        let iss = self.iss()?;
        Ok(ZkLoginPublicIdentifier {
            iss,
            address_seed: self.address_seed.clone(),
        })
    }
}

/// A claim of the iss in a zklogin proof
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// zklogin-claim = string u8
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct ZkLoginClaim {
    pub value: String,
    pub index_mod_4: u8,
}

#[derive(Debug)]
pub struct InvalidZkLoginClaimError(String);

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
impl InvalidZkLoginClaimError {
    fn new<T: Into<String>>(err: T) -> Self {
        Self(err.into())
    }
}

impl std::fmt::Display for InvalidZkLoginClaimError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid zklogin claim: {}", self.0)
    }
}

impl std::error::Error for InvalidZkLoginClaimError {}

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
impl ZkLoginClaim {
    fn verify_extended_claim(
        &self,
        expected_key: &str,
    ) -> Result<String, InvalidZkLoginClaimError> {
        /// Map a base64 string to a bit array by taking each char's index and convert it to binary form with one bit per u8
        /// element in the output. Returns InvalidZkLoginClaimError if one of the characters is not in the base64 charset.
        fn base64_to_bitarray(input: &str) -> Result<Vec<u8>, InvalidZkLoginClaimError> {
            use itertools::Itertools;

            const BASE64_URL_CHARSET: &str =
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

            input
                .chars()
                .map(|c| {
                    BASE64_URL_CHARSET
                        .find(c)
                        .map(|index| index as u8)
                        .map(|index| (0..6).rev().map(move |i| (index >> i) & 1))
                        .ok_or_else(|| {
                            InvalidZkLoginClaimError::new("base64_to_bitarry invalid input")
                        })
                })
                .flatten_ok()
                .collect()
        }

        /// Convert a bitarray (each bit is represented by a u8) to a byte array by taking each 8 bits as a
        /// byte in big-endian format.
        fn bitarray_to_bytearray(bits: &[u8]) -> Result<Vec<u8>, InvalidZkLoginClaimError> {
            if bits.len() % 8 != 0 {
                return Err(InvalidZkLoginClaimError::new(
                    "bitarray_to_bytearray invalid input",
                ));
            }
            Ok(bits
                .chunks(8)
                .map(|chunk| {
                    let mut byte = 0u8;
                    for (i, bit) in chunk.iter().rev().enumerate() {
                        byte |= bit << i;
                    }
                    byte
                })
                .collect())
        }

        /// Parse the base64 string, add paddings based on offset, and convert to a bytearray.
        fn decode_base64_url(
            s: &str,
            index_mod_4: &u8,
        ) -> Result<String, InvalidZkLoginClaimError> {
            if s.len() < 2 {
                return Err(InvalidZkLoginClaimError::new(
                    "Base64 string smaller than 2",
                ));
            }
            let mut bits = base64_to_bitarray(s)?;
            match index_mod_4 {
                0 => {}
                1 => {
                    bits.drain(..2);
                }
                2 => {
                    bits.drain(..4);
                }
                _ => {
                    return Err(InvalidZkLoginClaimError::new("Invalid first_char_offset"));
                }
            }

            let last_char_offset = (index_mod_4 + s.len() as u8 - 1) % 4;
            match last_char_offset {
                3 => {}
                2 => {
                    bits.drain(bits.len() - 2..);
                }
                1 => {
                    bits.drain(bits.len() - 4..);
                }
                _ => {
                    return Err(InvalidZkLoginClaimError::new("Invalid last_char_offset"));
                }
            }

            if bits.len() % 8 != 0 {
                return Err(InvalidZkLoginClaimError::new("Invalid bits length"));
            }

            Ok(std::str::from_utf8(&bitarray_to_bytearray(&bits)?)
                .map_err(|_| InvalidZkLoginClaimError::new("Invalid UTF8 string"))?
                .to_owned())
        }

        let extended_claim = decode_base64_url(&self.value, &self.index_mod_4)?;

        // Last character of each extracted_claim must be '}' or ','
        if !(extended_claim.ends_with('}') || extended_claim.ends_with(',')) {
            return Err(InvalidZkLoginClaimError::new("Invalid extended claim"));
        }

        let json_str = format!("{{{}}}", &extended_claim[..extended_claim.len() - 1]);

        serde_json::from_str::<serde_json::Value>(&json_str)
            .map_err(|e| InvalidZkLoginClaimError::new(e.to_string()))?
            .as_object_mut()
            .and_then(|o| o.get_mut(expected_key))
            .map(serde_json::Value::take)
            .and_then(|v| match v {
                serde_json::Value::String(s) => Some(s),
                _ => None,
            })
            .ok_or_else(|| InvalidZkLoginClaimError::new("invalid extended claim"))
    }
}

/// A zklogin groth16 proof
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// zklogin-proof = circom-g1 circom-g2 circom-g1
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct ZkLoginProof {
    pub a: CircomG1,
    pub b: CircomG2,
    pub c: CircomG1,
}

/// A G1 point
///
/// This represents the canonical decimal representation of the projective coordinates in Fq.
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// circom-g1 = %x03 3(bn254-field-element)
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct CircomG1(pub [Bn254FieldElement; 3]);

/// A G2 point
///
/// This represents the canonical decimal representation of the coefficients of the projective
/// coordinates in Fq2.
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// circom-g2 = %x03 3(%x02 2(bn254-field-element))
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct CircomG2(pub [[Bn254FieldElement; 2]; 3]);

/// Public Key equivalent for Zklogin authenticators
///
/// A `ZkLoginPublicIdentifier` is the equivalent of a public key for other account authenticators,
/// and contains the information required to derive the onchain account [`Address`] for a Zklogin
/// authenticator.
///
/// ## Note
///
/// Due to a historical bug that was introduced in the Sui Typescript SDK when the zklogin
/// authenticator was first introduced, there are now possibly two "valid" addresses for each
/// zklogin authenticator depending on the bit-pattern of the `address_seed` value.
///
/// The original bug incorrectly derived a zklogin's address by stripping any leading
/// zero-bytes that could have been present in the 32-byte length `address_seed` value prior to
/// hashing, leading to a different derived address. This incorrectly derived address was
/// presented to users of various wallets, leading them to sending funds to these addresses
/// that they couldn't access. Instead of letting these users lose any assets that were sent to
/// these addresses, the Sui network decided to change the protocol to allow for a zklogin
/// authenticator who's `address_seed` value had leading zero-bytes be authorized to sign for
/// both the addresses derived from both the unpadded and padded `address_seed` value.
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// zklogin-public-identifier-bcs = bytes ; where the contents are defined by
///                                       ; <zklogin-public-identifier>
///
/// zklogin-public-identifier = zklogin-public-identifier-iss
///                             address-seed
///
/// zklogin-public-identifier-unpadded = zklogin-public-identifier-iss
///                                      address-seed-unpadded
///
/// ; The iss, or issuer, is a utf8 string that is less than 255 bytes long
/// ; and is serialized with the iss's length in bytes as a u8 followed by
/// ; the bytes of the iss
/// zklogin-public-identifier-iss = u8 *255(OCTET)
///
/// ; A Bn254FieldElement serialized as a 32-byte big-endian value
/// address-seed = 32(OCTET)
///
/// ; A Bn254FieldElement serialized as a 32-byte big-endian value
/// ; with any leading zero bytes stripped
/// address-seed-unpadded = %x00 / %x01-ff *31(OCTET)
/// ```
///
/// [`Address`]: crate::Address
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct ZkLoginPublicIdentifier {
    iss: String,
    address_seed: Bn254FieldElement,
}

impl ZkLoginPublicIdentifier {
    pub fn new(iss: String, address_seed: Bn254FieldElement) -> Option<Self> {
        if iss.len() > 255 {
            None
        } else {
            Some(Self { iss, address_seed })
        }
    }

    pub fn iss(&self) -> &str {
        &self.iss
    }

    pub fn address_seed(&self) -> &Bn254FieldElement {
        &self.address_seed
    }
}

/// A JSON Web Key
///
/// Struct that contains info for a JWK. A list of them for different kids can
/// be retrieved from the JWK endpoint (e.g. <https://www.googleapis.com/oauth2/v3/certs>).
/// The JWK is used to verify the JWT token.
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// jwk = string string string string
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct Jwk {
    /// Key type parameter, <https://datatracker.ietf.org/doc/html/rfc7517#section-4.1>
    pub kty: String,

    /// RSA public exponent, <https://datatracker.ietf.org/doc/html/rfc7517#section-9.3>
    pub e: String,

    /// RSA modulus, <https://datatracker.ietf.org/doc/html/rfc7517#section-9.3>
    pub n: String,

    /// Algorithm parameter, <https://datatracker.ietf.org/doc/html/rfc7517#section-4.4>
    pub alg: String,
}

/// Key to uniquely identify a JWK
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// jwk-id = string string
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct JwkId {
    /// The issuer or identity of the OIDC provider.
    pub iss: String,

    /// A key id use to uniquely identify a key from an OIDC provider.
    pub kid: String,
}

/// A point on the BN254 elliptic curve.
///
/// This is a 32-byte, or 256-bit, value that is generally represented as radix10 when a
/// human-readable display format is needed, and is represented as a 32-byte big-endian value while
/// in memory.
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// bn254-field-element = *DIGIT ; which is then interpreted as a radix10 encoded 32-byte value
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct Bn254FieldElement([u8; 32]);

impl Bn254FieldElement {
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub const fn from_str_radix_10(s: &str) -> Result<Self, Bn254FieldElementParseError> {
        let u256 = match U256::from_str_radix(s, 10) {
            Ok(u256) => u256,
            Err(e) => return Err(Bn254FieldElementParseError(e)),
        };
        let be = u256.to_be();
        Ok(Self(*be.digits()))
    }

    pub fn unpadded(&self) -> &[u8] {
        let mut buf = self.0.as_slice();

        while !buf.is_empty() && buf[0] == 0 {
            buf = &buf[1..];
        }

        // If the value is '0' then just return a slice of length 1 of the final byte
        if buf.is_empty() {
            &self.0[31..]
        } else {
            buf
        }
    }

    pub fn padded(&self) -> &[u8] {
        &self.0
    }
}

impl std::fmt::Display for Bn254FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let u256 = U256::from_be(U256::from_digits(self.0));
        let radix10 = u256.to_str_radix(10);
        f.write_str(&radix10)
    }
}

#[derive(Debug)]
pub struct Bn254FieldElementParseError(bnum::errors::ParseIntError);

impl std::fmt::Display for Bn254FieldElementParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unable to parse radix10 encoded value {}", self.0)
    }
}

impl std::error::Error for Bn254FieldElementParseError {}

impl std::str::FromStr for Bn254FieldElement {
    type Err = Bn254FieldElementParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let u256 = U256::from_str_radix(s, 10).map_err(Bn254FieldElementParseError)?;
        let be = u256.to_be();
        Ok(Self(*be.digits()))
    }
}

#[cfg(test)]
mod test {
    use super::Bn254FieldElement;
    use num_bigint::BigUint;
    use proptest::prelude::*;
    use std::str::FromStr;
    use test_strategy::proptest;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    #[test]
    fn unpadded_slice() {
        let seed = Bn254FieldElement([0; 32]);
        let zero: [u8; 1] = [0];
        assert_eq!(seed.unpadded(), zero.as_slice());

        let mut seed = Bn254FieldElement([1; 32]);
        seed.0[0] = 0;
        assert_eq!(seed.unpadded(), [1; 31].as_slice());
    }

    #[proptest]
    fn dont_crash_on_large_inputs(
        #[strategy(proptest::collection::vec(any::<u8>(), 33..1024))] bytes: Vec<u8>,
    ) {
        let big_int = BigUint::from_bytes_be(&bytes);
        let radix10 = big_int.to_str_radix(10);

        // doesn't crash
        let _ = Bn254FieldElement::from_str(&radix10);
    }

    #[proptest]
    fn valid_address_seeds(
        #[strategy(proptest::collection::vec(any::<u8>(), 1..=32))] bytes: Vec<u8>,
    ) {
        let big_int = BigUint::from_bytes_be(&bytes);
        let radix10 = big_int.to_str_radix(10);

        let seed = Bn254FieldElement::from_str(&radix10).unwrap();
        assert_eq!(radix10, seed.to_string());
        // Ensure unpadded doesn't crash
        seed.unpadded();
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
mod serialization {
    use crate::SignatureScheme;

    use super::*;
    use serde::Deserialize;
    use serde::Deserializer;
    use serde::Serialize;
    use serde::Serializer;
    use serde_with::Bytes;
    use serde_with::DeserializeAs;
    use serde_with::SerializeAs;
    use std::borrow::Cow;

    // Serialized format is: iss_bytes_len || iss_bytes || padded_32_byte_address_seed.
    impl Serialize for ZkLoginPublicIdentifier {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                #[derive(serde_derive::Serialize)]
                struct Readable<'a> {
                    iss: &'a str,
                    address_seed: &'a Bn254FieldElement,
                }
                let readable = Readable {
                    iss: &self.iss,
                    address_seed: &self.address_seed,
                };
                readable.serialize(serializer)
            } else {
                let mut buf = Vec::new();
                let iss_bytes = self.iss.as_bytes();
                buf.push(iss_bytes.len() as u8);
                buf.extend(iss_bytes);

                buf.extend(&self.address_seed.0);

                serializer.serialize_bytes(&buf)
            }
        }
    }

    impl<'de> Deserialize<'de> for ZkLoginPublicIdentifier {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                #[derive(serde_derive::Deserialize)]
                struct Readable {
                    iss: String,
                    address_seed: Bn254FieldElement,
                }

                let Readable { iss, address_seed } = Deserialize::deserialize(deserializer)?;
                Self::new(iss, address_seed)
                    .ok_or_else(|| serde::de::Error::custom("invalid zklogin public identifier"))
            } else {
                let bytes: Cow<'de, [u8]> = Bytes::deserialize_as(deserializer)?;
                let iss_len = *bytes
                    .first()
                    .ok_or_else(|| serde::de::Error::custom("invalid zklogin public identifier"))?;
                let iss_bytes = bytes
                    .get(1..(1 + iss_len as usize))
                    .ok_or_else(|| serde::de::Error::custom("invalid zklogin public identifier"))?;
                let iss = std::str::from_utf8(iss_bytes).map_err(serde::de::Error::custom)?;
                let address_seed_bytes = bytes
                    .get((1 + iss_len as usize)..)
                    .ok_or_else(|| serde::de::Error::custom("invalid zklogin public identifier"))?;

                let address_seed = <[u8; 32]>::try_from(address_seed_bytes)
                    .map_err(serde::de::Error::custom)
                    .map(Bn254FieldElement)?;

                Self::new(iss.into(), address_seed)
                    .ok_or_else(|| serde::de::Error::custom("invalid zklogin public identifier"))
            }
        }
    }

    #[derive(serde_derive::Serialize)]
    struct AuthenticatorRef<'a> {
        inputs: &'a ZkLoginInputs,
        #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
        max_epoch: EpochId,
        signature: &'a SimpleSignature,
    }

    #[derive(serde_derive::Deserialize)]
    struct Authenticator {
        inputs: ZkLoginInputs,
        #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
        max_epoch: EpochId,
        signature: SimpleSignature,
    }

    impl Serialize for ZkLoginAuthenticator {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                let authenticator_ref = AuthenticatorRef {
                    inputs: &self.inputs,
                    max_epoch: self.max_epoch,
                    signature: &self.signature,
                };

                authenticator_ref.serialize(serializer)
            } else {
                let bytes = self.to_bytes();
                serializer.serialize_bytes(&bytes)
            }
        }
    }

    impl<'de> Deserialize<'de> for ZkLoginAuthenticator {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                let Authenticator {
                    inputs,
                    max_epoch,
                    signature,
                } = Authenticator::deserialize(deserializer)?;
                Ok(Self {
                    inputs,
                    max_epoch,
                    signature,
                })
            } else {
                let bytes: Cow<'de, [u8]> = Bytes::deserialize_as(deserializer)?;
                Self::from_serialized_bytes(bytes)
            }
        }
    }

    impl ZkLoginAuthenticator {
        pub(crate) fn to_bytes(&self) -> Vec<u8> {
            let authenticator_ref = AuthenticatorRef {
                inputs: &self.inputs,
                max_epoch: self.max_epoch,
                signature: &self.signature,
            };

            let mut buf = Vec::new();
            buf.push(SignatureScheme::ZkLogin as u8);

            bcs::serialize_into(&mut buf, &authenticator_ref).expect("serialization cannot fail");
            buf
        }

        pub(crate) fn from_serialized_bytes<T: AsRef<[u8]>, E: serde::de::Error>(
            bytes: T,
        ) -> Result<Self, E> {
            let bytes = bytes.as_ref();
            let flag = SignatureScheme::from_byte(
                *bytes
                    .first()
                    .ok_or_else(|| serde::de::Error::custom("missing signature scheme falg"))?,
            )
            .map_err(serde::de::Error::custom)?;
            if flag != SignatureScheme::ZkLogin {
                return Err(serde::de::Error::custom("invalid zklogin flag"));
            }
            let bcs_bytes = &bytes[1..];

            let Authenticator {
                inputs,
                max_epoch,
                signature,
            } = bcs::from_bytes(bcs_bytes).map_err(serde::de::Error::custom)?;
            Ok(Self {
                inputs,
                max_epoch,
                signature,
            })
        }
    }

    // AddressSeed's serialized format is as a radix10 encoded string
    impl Serialize for Bn254FieldElement {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serde_with::DisplayFromStr::serialize_as(self, serializer)
        }
    }

    impl<'de> Deserialize<'de> for Bn254FieldElement {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            serde_with::DisplayFromStr::deserialize_as(deserializer)
        }
    }

    impl Serialize for CircomG1 {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            use serde::ser::SerializeSeq;
            let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
            for element in &self.0 {
                seq.serialize_element(element)?;
            }
            seq.end()
        }
    }

    impl<'de> Deserialize<'de> for CircomG1 {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let inner = <Vec<_>>::deserialize(deserializer)?;
            Ok(Self(inner.try_into().map_err(|_| {
                serde::de::Error::custom("expected array of length 3")
            })?))
        }
    }

    impl Serialize for CircomG2 {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            use serde::ser::SerializeSeq;

            struct Inner<'a>(&'a [Bn254FieldElement; 2]);

            impl Serialize for Inner<'_> {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
                    for element in self.0 {
                        seq.serialize_element(element)?;
                    }
                    seq.end()
                }
            }

            let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
            for element in &self.0 {
                seq.serialize_element(&Inner(element))?;
            }
            seq.end()
        }
    }

    impl<'de> Deserialize<'de> for CircomG2 {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let vecs = <Vec<Vec<Bn254FieldElement>>>::deserialize(deserializer)?;
            let mut inner: [[Bn254FieldElement; 2]; 3] = Default::default();

            if vecs.len() != 3 {
                return Err(serde::de::Error::custom(
                    "vector of three vectors each being a vector of two strings",
                ));
            }

            for (i, v) in vecs.into_iter().enumerate() {
                if v.len() != 2 {
                    return Err(serde::de::Error::custom(
                        "vector of three vectors each being a vector of two strings",
                    ));
                }

                for (j, point) in v.into_iter().enumerate() {
                    inner[i][j] = point;
                }
            }

            Ok(Self(inner))
        }
    }
}
