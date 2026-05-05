//! Encoding and decoding helpers for the Sui `suiprivkey` Bech32 format.
//!
//! The format mirrors the one produced by the Sui CLI and the `sui-types`
//! crate: a 33-byte payload of `flag || private_key` encoded as a Bech32
//! (BIP-173) string with the human-readable part `suiprivkey`. The leading
//! flag byte is the `SignatureScheme` flag for the contained key (`0x00` for
//! Ed25519, `0x01` for Secp256k1, `0x02` for Secp256r1).
//!
//! These helpers are kept `pub(crate)` on purpose. Callers reach the format
//! through strongly-typed wrappers like `Ed25519PrivateKey::from_suiprivkey`
//! or `SimpleKeypair::from_suiprivkey` to avoid handing raw key bytes back as
//! a `Vec<u8>` at the public boundary.

use bech32::Bech32;
use bech32::Hrp;
use bech32::primitives::decode::CheckedHrpstring;
use sui_sdk_types::SignatureScheme;

use crate::SignatureError;

/// The human-readable part of the `suiprivkey` Bech32 encoding.
const HRP: &str = "suiprivkey";

fn hrp() -> Hrp {
    // "suiprivkey" is a valid Bech32 HRP (lowercase ASCII, length 10).
    Hrp::parse(HRP).expect("`suiprivkey` is a valid Bech32 HRP")
}

/// Encode a `flag || private_key` payload as a Bech32 `suiprivkey` string.
pub(crate) fn encode(scheme: SignatureScheme, key: &[u8]) -> Result<String, SignatureError> {
    let mut payload = Vec::with_capacity(1 + key.len());
    payload.push(scheme.to_u8());
    payload.extend_from_slice(key);
    bech32::encode::<Bech32>(hrp(), &payload)
        .map_err(|e| SignatureError::from_source(format!("bech32 encode failed: {e}")))
}

/// Decode a Bech32 `suiprivkey` string into its scheme flag and key bytes.
///
/// The BIP-173 checksum is validated strictly; Bech32m-checksummed strings are
/// rejected. The returned key bytes are everything after the flag byte and
/// are not validated against the scheme — the caller is responsible for
/// verifying the length and constructing a scheme-specific key from them.
pub(crate) fn decode(s: &str) -> Result<(SignatureScheme, Vec<u8>), SignatureError> {
    let parsed = CheckedHrpstring::new::<Bech32>(s)
        .map_err(|e| SignatureError::from_source(format!("invalid suiprivkey string: {e}")))?;

    if parsed.hrp() != hrp() {
        return Err(SignatureError::from_source(format!(
            "expected `{HRP}` human-readable part",
        )));
    }

    let bytes: Vec<u8> = parsed.byte_iter().collect();
    let (flag, key) = bytes
        .split_first()
        .ok_or_else(|| SignatureError::from_source("suipriv payload is empty"))?;
    let scheme = SignatureScheme::from_byte(*flag)
        .map_err(|e| SignatureError::from_source(format!("invalid suipriv scheme flag: {e}")))?;
    Ok((scheme, key.to_vec()))
}
