# [0.0.3] - 2025-03-20

- Update to use version 0.0.3 of `sui-sdk-types`

# [0.0.2] - 2025-01-06

## Added

- Added support for multisig verification and aggregation ([#25])
- Added blanket implementation for SuiSigner and SuiVerifier ([`bc481a1`])
- Added support for der and pem format for public and private keys ([`df32a46`])
- Added a `SimpleKeypair` type which could be either an ed25519, secp256k1, or secp256r1 keypair ([`8d64c06`])
- Added support for verifying passkey authenticators ([#81])

[#25]: https://github.com/mystenlabs/sui-rust-sdk/pull/25
[`bc481a1`]: https://github.com/mystenlabs/sui-rust-sdk/commit/bc481a1ea156e6ccb528b5b49e62a511be5ba60a
[`df32a46`]: https://github.com/mystenlabs/sui-rust-sdk/commit/df32a46bfbecbbbf4ec7e9c1974eef0916ccd359
[`8d64c06`]: https://github.com/mystenlabs/sui-rust-sdk/commit/8d64c06628b9494c674c27158ce74036fe45080e
[#81]: https://github.com/MystenLabs/sui-rust-sdk/pull/81

# [0.0.1] - 2024-09-25

Initial release

[0.0.3]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-crypto-0.0.3
[0.0.2]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-crypto-0.0.2
[0.0.1]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-crypto-0.0.1
