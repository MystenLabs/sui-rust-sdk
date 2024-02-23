mod bls12381;
mod ed25519;
mod secp256k1;
mod secp256r1;
mod signature;

pub use bls12381::{Bls12381PrivateKey, Bls12381PublicKey, Bls12381Signature};
pub use ed25519::{Ed25519PrivateKey, Ed25519PublicKey, Ed25519Signature};
pub use secp256k1::{Secp256k1PrivateKey, Secp256k1PublicKey, Secp256k1Signature};
pub use secp256r1::{Secp256r1PrivateKey, Secp256r1PublicKey, Secp256r1Signature};
pub use signature::{SignatureScheme, SimpleSignature, UserSignature};

//
// Implement various base64 fixed-size array helpers
//

/// Utility for calculating base64 encoding lenghths.
///
/// In the Base64 encoding each character is used to represent 6 bits (log2(64) = 6). This means
/// that 4 characters are used to represnet 4*6 = 24 bits = 3 bytes. So you need 4*(`n`/3)
/// characters in order to represent `n` bytes, and this needs to be rounded up to a multiple of 4.
/// The number of unused padding characters resulting from the rounding will be 0, 1, 2, or 3.
const fn base64_encoded_length(len: usize) -> usize {
    ((4 * len / 3) + 3) & !3
}

macro_rules! impl_base64_helper {
    ($base:ident, $display:ident, $fromstr:ident, $array_length:literal) => {
        struct $base;

        impl $base {
            const LENGTH: usize = $array_length;
            const ENCODED_LENGTH: usize = base64_encoded_length(Self::LENGTH);
        }

        struct $display<'a>(&'a [u8; $base::LENGTH]);

        impl<'a> std::fmt::Display for $display<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut buf = [0; $base::ENCODED_LENGTH];
                let encoded =
                    <base64ct::Base64 as base64ct::Encoding>::encode(self.0, &mut buf).unwrap();
                f.write_str(encoded)
            }
        }

        struct $fromstr([u8; $base::LENGTH]);

        impl std::str::FromStr for $fromstr {
            type Err = base64ct::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let mut buf = [0; $base::LENGTH];
                let decoded = <base64ct::Base64 as base64ct::Encoding>::decode(s, &mut buf)?;
                assert_eq!(decoded.len(), $base::LENGTH);
                Ok(Self(buf))
            }
        }

        #[cfg(feature = "serde")]
        #[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
        impl serde_with::SerializeAs<[u8; Self::LENGTH]> for $base {
            fn serialize_as<S>(
                source: &[u8; Self::LENGTH],
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let display = $display(source);
                serde_with::DisplayFromStr::serialize_as(&display, serializer)
            }
        }

        #[cfg(feature = "serde")]
        #[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
        impl<'de> serde_with::DeserializeAs<'de, [u8; Self::LENGTH]> for $base {
            fn deserialize_as<D>(deserializer: D) -> Result<[u8; Self::LENGTH], D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let array: $fromstr = serde_with::DisplayFromStr::deserialize_as(deserializer)?;
                Ok(array.0)
            }
        }

        // TODO add tests
    };
}

impl_base64_helper!(Base64Array32, Base64Display32, Base64FromStr32, 32);
impl_base64_helper!(Base64Array33, Base64Display33, Base64FromStr33, 33);
impl_base64_helper!(Base64Array48, Base64Display48, Base64FromStr48, 48);
impl_base64_helper!(Base64Array64, Base64Display64, Base64FromStr64, 64);
impl_base64_helper!(Base64Array96, Base64Display96, Base64FromStr96, 96);
