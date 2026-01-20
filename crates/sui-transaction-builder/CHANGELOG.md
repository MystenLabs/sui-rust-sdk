# [0.2.2] - 2026-01-20

## Changed
- [`da176c95`] add `intents` feature to allow for enabling intents and
  transaction resolution, which is dependent on communicating with a fullnode,
  optional for those users who only want to use this crate for ofline
  transaction construction.

[`da176c95`]: https://github.com/mystenlabs/sui-rust-sdk/commit/da176c95

# [0.2.1] - 2026-01-07

## Fixed
- [#188] deterministically order inputs in built transaction

[#188]: https://github.com/MystenLabs/sui-rust-sdk/pull/188

# [0.2.0] - 2026-01-05

## Breaking
- [#183] Rework `TransactionBuilder` to be more opaque [#183]
- [#183] Introduce transaction `Intent`s similar to intents in the TS sdk.
  For now the `Intent` trait is private to this crate and only intents defined
  in this crate are supported. This will allow us to iterate on the api till
  its safer and more ergonomic to use.

[#183]: https://github.com/MystenLabs/sui-rust-sdk/pull/183

[0.2.1]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-transaction-builder-0.2.1
[0.2.0]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-transaction-builder-0.2.0
