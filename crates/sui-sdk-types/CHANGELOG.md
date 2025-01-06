# [0.0.2] - 2025-01-06

## Added

- Added `proptest::Arbitrary` impls via the `proptest` feature [`6918fd8`]
- Added From<StructTag> impl for TypeTag [#77]

## Changed

- Update the passkey challenge format to use the same signing message as other key types ([`c5a25ce`])
- Flattened the `types` module into the top-level ([`dc54c46`])
- Folded the `EffectsObjectChange` type into the `ChangedObject` struct ([`aa546ca`])

## Removed

- Removed the `unresolved` module and moved it to the `sui-transaction-builder` crate ([`d965897`])
- Removed the `schemars` feature ([`bc6dd37`])

[`c5a25ce`]: https://github.com/mystenlabs/sui-rust-sdk/commit/c5a25ce356a8cbe42ddcc6ec6bab380007790b44
[`6918fd8`]: https://github.com/mystenlabs/sui-rust-sdk/commit/6918fd88d40734b8c15fb5c519e9a40aec53eb74
[#77]: https://github.com/mystenlabs/sui-rust-sdk/pull/77
[`d965897`]: https://github.com/mystenlabs/sui-rust-sdk/commit/d9658978a4c6e928d036fbedaab9326d5e28de87
[`dc54c46`]: https://github.com/mystenlabs/sui-rust-sdk/commit/dc54c469f9d006f02d82ec5781d73e8e09ae26ae
[`aa546ca`]: https://github.com/mystenlabs/sui-rust-sdk/commit/aa546ca91249932da3f8e3d55ba6e52e40cd8929
[`bc6dd37`]: https://github.com/mystenlabs/sui-rust-sdk/commit/bc6dd3732973ed3c1c3ae811a818fc8504a99f0b

# [0.0.1] - 2024-09-25

Initial release

[0.0.2]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-sdk-types-0.0.2
[0.0.1]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-sdk-types-0.0.1
