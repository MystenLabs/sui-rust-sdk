//! Encoding and decoding helpers for the serialized Sui private key formats.
//!
//! Both formats carry the same payload of `flag || private_key`, where the
//! leading flag byte is the `SignatureScheme` flag for the contained key
//! (`0x00` for Ed25519, `0x01` for Secp256k1, `0x02` for Secp256r1). The
//! payload has two wire encodings, mirroring the Sui CLI and the `sui-types`
//! crate:
//!
//! - `bech32`: a Bech32 (BIP-173) string with the human-readable part
//!   `suiprivkey`, produced by `sui keytool generate` and `sui keytool
//!   export`.
//! - `base64`: a plain Base64 string, the legacy encoding used for entries
//!   of the Sui CLI's `sui.keystore` file and by older versions of `sui
//!   keytool`.
//!
//! These helpers are kept `pub(crate)` on purpose. Callers reach the formats
//! through strongly-typed wrappers like `Ed25519PrivateKey::from_suiprivkey`
//! or `SimpleKeypair::from_base64` to avoid handing raw key bytes back as a
//! `Vec<u8>` at the public boundary.

use sui_sdk_types::SignatureScheme;

use crate::SignatureError;

/// Join a scheme flag and key bytes into a `flag || private_key` payload.
fn join_payload(scheme: SignatureScheme, key: &[u8]) -> Vec<u8> {
    let mut payload = Vec::with_capacity(1 + key.len());
    payload.push(scheme.to_u8());
    payload.extend_from_slice(key);
    payload
}

/// Split a `flag || private_key` payload into its scheme flag and key bytes.
///
/// The returned key bytes are not validated against the scheme — the caller
/// is responsible for verifying the length and constructing a scheme-specific
/// key from them.
fn split_payload(bytes: &[u8]) -> Result<(SignatureScheme, Vec<u8>), SignatureError> {
    let (flag, key) = bytes
        .split_first()
        .ok_or_else(|| SignatureError::from_source("private key payload is empty"))?;
    let scheme = SignatureScheme::from_byte(*flag).map_err(|e| {
        SignatureError::from_source(format!("invalid private key scheme flag: {e}"))
    })?;
    Ok((scheme, key.to_vec()))
}

/// The human-readable part of the `suiprivkey` Bech32 encoding.
#[cfg(feature = "bech32")]
const HRP: &str = "suiprivkey";

#[cfg(feature = "bech32")]
fn hrp() -> bech32::Hrp {
    // "suiprivkey" is a valid Bech32 HRP (lowercase ASCII, length 10).
    bech32::Hrp::parse(HRP).expect("`suiprivkey` is a valid Bech32 HRP")
}

/// Encode a `flag || private_key` payload as a Bech32 `suiprivkey` string.
#[cfg(feature = "bech32")]
pub(crate) fn encode(scheme: SignatureScheme, key: &[u8]) -> Result<String, SignatureError> {
    bech32::encode::<bech32::Bech32>(hrp(), &join_payload(scheme, key))
        .map_err(|e| SignatureError::from_source(format!("bech32 encode failed: {e}")))
}

/// Decode a Bech32 `suiprivkey` string into its scheme flag and key bytes.
///
/// The BIP-173 checksum is validated strictly; Bech32m-checksummed strings are
/// rejected.
#[cfg(feature = "bech32")]
pub(crate) fn decode(s: &str) -> Result<(SignatureScheme, Vec<u8>), SignatureError> {
    let parsed = bech32::primitives::decode::CheckedHrpstring::new::<bech32::Bech32>(s)
        .map_err(|e| SignatureError::from_source(format!("invalid suiprivkey string: {e}")))?;

    if parsed.hrp() != hrp() {
        return Err(SignatureError::from_source(format!(
            "expected `{HRP}` human-readable part",
        )));
    }

    let bytes: Vec<u8> = parsed.byte_iter().collect();
    split_payload(&bytes)
}

/// Encode a `flag || private_key` payload as a legacy Base64 keystore string.
pub(crate) fn encode_base64(scheme: SignatureScheme, key: &[u8]) -> String {
    use base64ct::Encoding;

    base64ct::Base64::encode_string(&join_payload(scheme, key))
}

/// Decode a legacy Base64 keystore string into its scheme flag and key bytes.
pub(crate) fn decode_base64(s: &str) -> Result<(SignatureScheme, Vec<u8>), SignatureError> {
    use base64ct::Encoding;

    let bytes = base64ct::Base64::decode_vec(s).map_err(|e| {
        SignatureError::from_source(format!("invalid base64 private key string: {e}"))
    })?;
    split_payload(&bytes)
}
