# [0.2.2] - 2026-01-20

## Added
- [#202] add support for TransactionKind::ProgrammableSystemTransaction
- [#204] add support for EndOfEpochTransactionKind::WriteAccumulatorStorageCost

## Fixed
- [#190] fix bcs serialized format of Reservation for FundsWithdrawal
  transaction input.

[#190]: https://github.com/MystenLabs/sui-rust-sdk/pull/190
[#202]: https://github.com/MystenLabs/sui-rust-sdk/pull/202
[#204]: https://github.com/MystenLabs/sui-rust-sdk/pull/204

# [0.2.1] - 2026-01-07

## Fixed
- [#186] fix ser/de of the efficient StructTag serialization used in Object
  serialization for the new address balance accumulator types.

[#186]: https://github.com/MystenLabs/sui-rust-sdk/pull/186

# [0.2.0] - 2026-01-05

## Breaking
- `Input::Shared` changed to support new `Mutability` enum instead of it being a boolean [#179]
- Reworked the CheckpointContents and CheckpointTransactionInfo types to support contents v2 [#180]

## Added
- Added support for address balances [#179]
- Added support for address aliases [#177]
- Added support for CheckpointContents V2 [#180]
- Added support for deserializing UserSignatures from base64 in human-readable formats [#182]

[#177]: https://github.com/MystenLabs/sui-rust-sdk/pull/177
[#179]: https://github.com/MystenLabs/sui-rust-sdk/pull/179
[#180]: https://github.com/MystenLabs/sui-rust-sdk/pull/180
[#182]: https://github.com/MystenLabs/sui-rust-sdk/pull/182

# [0.1.1] - 2025-12-11

## Added
- Added new move vm adapter error variants

# [0.1.0] - 2025-11-07

## Added
- Added `DisplayRegistryCreate` end of epoch transaction kind [`36187fdf`]
- Added `FromBcs` and `ToBcs` convenience traits [`08e2ec32`]

## Changed
- Updated to rust 2024 edition [#171]
- Made `StructTag` fields private [#175]

[`08e2ec32`]: https://github.com/mystenlabs/sui-rust-sdk/commit/08e2ec32
[`36187fdf`]: https://github.com/mystenlabs/sui-rust-sdk/commit/36187fdf
[#171]: https://github.com/MystenLabs/sui-rust-sdk/pull/171
[#175]: https://github.com/MystenLabs/sui-rust-sdk/pull/175

# [0.0.8] - 2025-10-03

## Added
- Helper for deriving `Address` from `UserSignautre` [#165]
- Implement From for String for Address, Digest and TypeTag [#150]
- Support for deriving addresses for derived objects [#145]
- Support for CheckpointCommitment::CheckpointArtifactsDigest [#126]

## Changed
- Renamed `from_*_unwrap` to `from_static` for Address and Digest [`8e80c8eb`]

[#165]: https://github.com/MystenLabs/sui-rust-sdk/pull/165
[#150]: https://github.com/MystenLabs/sui-rust-sdk/pull/150
[#145]: https://github.com/MystenLabs/sui-rust-sdk/pull/145
[`8e80c8eb`]: https://github.com/mystenlabs/sui-rust-sdk/commit/8e80c8eb
[#126]: https://github.com/MystenLabs/sui-rust-sdk/pull/126

# [0.0.7] - 2025-08-29

## Added

- Added const constructor for Identifier ([`c08b6b69`])
- Added const parsing constructors for Address and Digest ([`c5cc14b6`])
- Added wrapper around RoaringBitmap ([`8b9c14f0`])
- Introduced type safety with ZkLoginInputs ([`ceeee4a3`])

## Changed

- Marked public enums with non_exhaustive ([`fd36eb13`])
- Renamed shared -> consensus ([`cab42748`])
- Removed digest wrappers and use Digest everywhere ([`2e24c6d6`])
- Removed ObjectId type in favor of Address ([`358569a7`])
- Updated UnchangedSharedObjectKind to match new names in sui repo ([`d9719506`])

## Fixed

- Fixed typos ([`4c426996`])

[`c08b6b69`]: https://github.com/mystenlabs/sui-rust-sdk/commit/c08b6b69
[`c5cc14b6`]: https://github.com/mystenlabs/sui-rust-sdk/commit/c5cc14b6
[`fd36eb13`]: https://github.com/mystenlabs/sui-rust-sdk/commit/fd36eb13
[`8b9c14f0`]: https://github.com/mystenlabs/sui-rust-sdk/commit/8b9c14f0
[`ceeee4a3`]: https://github.com/mystenlabs/sui-rust-sdk/commit/ceeee4a3
[`cab42748`]: https://github.com/mystenlabs/sui-rust-sdk/commit/cab42748
[`2e24c6d6`]: https://github.com/mystenlabs/sui-rust-sdk/commit/2e24c6d6
[`358569a7`]: https://github.com/mystenlabs/sui-rust-sdk/commit/358569a7
[`d9719506`]: https://github.com/mystenlabs/sui-rust-sdk/commit/d9719506
[`4c426996`]: https://github.com/mystenlabs/sui-rust-sdk/commit/4c426996

# [0.0.6] - 2025-07-16

## Added

- Added EndOfEpochTransactionKind::CoinRegistryCreate [#117]

## Removed

- Removed bespoke json serde impls for various types [`0c383a17`]

[#117]: https://github.com/MystenLabs/sui-rust-sdk/pull/117
[`0c383a17`]: https://github.com/mystenlabs/sui-rust-sdk/commit/0c383a177f80ac44876e70367c51b1ab3c5ea043

# [0.0.5] - 2025-06-12

## Added

- Added support for various new variants and types.
- Added support for deriving ZkLoginPublicIdentifier from ZkLoginInputs ([`ce2b6b4d`])
- Added support for passkeys in multisigs ([`5b61c62a`])

[`ce2b6b4d`]: https://github.com/mystenlabs/sui-rust-sdk/commit/ce2b6b4d149c44d08bc89a1bf051762dfdb30e9e
[`5b61c62a`]: https://github.com/mystenlabs/sui-rust-sdk/commit/5b61c62acdb36a11ee7df531f8e7f57ed841ae59

# [0.0.4] - 2025-03-31

## Added

- Added new `EndOfEpochTransactionKind::StoreExecutionTimeObservations` type [#105]

[#105]: https://github.com/MystenLabs/sui-rust-sdk/pull/105

# [0.0.3] - 2025-03-20

## Added

- Added `Object::as_struct` getter [#87]
- Added `ZkLoginPublicIdentifier::derive_address` with provides an iterator
  over the valid derived addresses [`a9a930d`]
- Added documentation for a number of types.
- Added support for the `ConsensusCommitPrologueV4` system transaction [`5e11579`]

## Changed

- Renamed `to_address` to `derive_address` for all authenticators [`2442379`]

[`2442379`]: https://github.com/mystenlabs/sui-rust-sdk/commit/2442379f19bdae8c560d9879ee291560a7cd2e1c
[`a9a930d`]: https://github.com/mystenlabs/sui-rust-sdk/commit/a9a930d9f8afbfc025f8978e317025798d225790
[`5e11579`]: https://github.com/mystenlabs/sui-rust-sdk/commit/5e11579031793f086178332219f5847ec94da0c4
[#87]: https://github.com/MystenLabs/sui-rust-sdk/pull/87

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

[0.2.2]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-sdk-types-0.2.2
[0.2.1]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-sdk-types-0.2.1
[0.2.0]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-sdk-types-0.2.0
[0.1.1]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-sdk-types-0.1.1
[0.1.0]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-sdk-types-0.1.0
[0.0.8]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-sdk-types-0.0.8
[0.0.7]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-sdk-types-0.0.7
[0.0.6]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-sdk-types-0.0.6
[0.0.5]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-sdk-types-0.0.5
[0.0.4]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-sdk-types-0.0.4
[0.0.3]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-sdk-types-0.0.3
[0.0.2]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-sdk-types-0.0.2
[0.0.1]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-sdk-types-0.0.1
