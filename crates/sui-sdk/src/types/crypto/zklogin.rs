use super::SimpleSignature;
use crate::types::checkpoint::EpochId;
use crate::types::u256::U256;

/// An zk login authenticator with all the necessary fields.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(test, derive(test_strategy::Arbitrary))]
pub struct ZkLoginAuthenticator {
    inputs: ZkLoginInputs,
    #[cfg_attr(feature = "schemars", schemars(with = "crate::_schemars::U64"))]
    max_epoch: EpochId,
    signature: SimpleSignature,
}

/// All inputs required for the zk login proof verification and other public inputs.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(test, derive(test_strategy::Arbitrary))]
pub struct ZkLoginInputs {
    proof_points: ZkLoginProof,
    iss_base64_details: Claim,
    header_base64: String,
    address_seed: Bn254FieldElement,
    // #[serde(skip)]
    // jwt_details: JwtDetails,
}

/// A claim consists of value and index_mod_4.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(test, derive(test_strategy::Arbitrary))]
pub struct Claim {
    value: String,
    index_mod_4: u8,
}

/// A structed of parsed JWT details, consists of kid, header, iss.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(test, derive(test_strategy::Arbitrary))]
pub struct JwtDetails {
    kid: String,
    header: String,
    iss: String,
}

/// The struct for zk login proof.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(test, derive(test_strategy::Arbitrary))]
pub struct ZkLoginProof {
    a: CircomG1,
    b: CircomG2,
    c: CircomG1,
}

/// A G1 point in BN254 serialized as a vector of three strings which is the canonical decimal
/// representation of the projective coordinates in Fq.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(test, derive(test_strategy::Arbitrary))]
pub struct CircomG1([Bn254FieldElement; 3]);

/// A G2 point in BN254 serialized as a vector of three vectors each being a vector of two strings
/// which are the canonical decimal representation of the coefficients of the projective coordinates
/// in Fq2.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(test, derive(test_strategy::Arbitrary))]
pub struct CircomG2([[Bn254FieldElement; 2]; 3]);

/// A wrapper struct to retrofit in [enum PublicKey] for zkLogin.
/// Useful to construct [struct MultiSigPublicKey].
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(test, derive(test_strategy::Arbitrary))]
//TODO ensure iss is less than 255 bytes long
pub struct ZkLoginPublicIdentifier {
    iss: String,
    address_seed: Bn254FieldElement,
}

impl ZkLoginPublicIdentifier {
    pub fn iss(&self) -> &str {
        &self.iss
    }

    pub fn address_seed(&self) -> &Bn254FieldElement {
        &self.address_seed
    }
}

/// Struct that contains info for a JWK. A list of them for different kids can
/// be retrieved from the JWK endpoint (e.g. <https://www.googleapis.com/oauth2/v3/certs>).
/// The JWK is used to verify the JWT token.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(test, derive(test_strategy::Arbitrary))]
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

/// Key to identify a JWK, consists of iss and kid.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(test, derive(test_strategy::Arbitrary))]
pub struct JwkId {
    /// iss string that identifies the OIDC provider.
    pub iss: String,
    /// kid string that identifies the JWK.
    pub kid: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(test, derive(test_strategy::Arbitrary))]
pub struct Bn254FieldElement(
    #[cfg_attr(feature = "schemars", schemars(with = "crate::_schemars::U256"))] [u8; 32],
);

impl Bn254FieldElement {
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
    use crate::types::SignatureScheme;

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
                Ok(Self { iss, address_seed })
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

                Ok(Self {
                    iss: iss.into(),
                    address_seed,
                })
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
            let authenticator_ref = AuthenticatorRef {
                inputs: &self.inputs,
                max_epoch: self.max_epoch,
                signature: &self.signature,
            };
            if serializer.is_human_readable() {
                authenticator_ref.serialize(serializer)
            } else {
                let mut buf = Vec::new();
                buf.push(SignatureScheme::ZkLogin as u8);

                bcs::serialize_into(&mut buf, &authenticator_ref)
                    .expect("serialization cannot fail");
                serializer.serialize_bytes(&buf)
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

            impl<'a> Serialize for Inner<'a> {
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
