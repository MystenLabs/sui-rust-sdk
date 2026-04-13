# [0.3.1] - 2026-04-13

## Added
- [#238] add a `Balance` intent for resolving `Balance<T>` directly from
  the transaction builder, alongside the existing coin intent. The
  `CoinWithBalance` intent is renamed to `Coin` with a backward-compatible
  type alias. Both intents share a resolver so that mixed `Coin` and
  `Balance` requests for the same coin type are accounted for together
- [#238] add `TransactionBuilder::coin` and `TransactionBuilder::balance`
  convenience methods for creating `Coin` and `Balance` intents without
  importing the intent types directly

## Changed
- [#238] prefer address balance over coins across all intent types. Any
  mix of `Coin` and `Balance` intents now takes the direct-withdrawal path
  whenever the sender's address balance covers the total, so coins are
  only touched when the address balance is insufficient. When the
  merge-and-split path is used and address balance contributes to the
  shortfall, the remainder is returned to the sender's address balance
  rather than destroyed

[#238]: https://github.com/MystenLabs/sui-rust-sdk/pull/238

# [0.3.0] - 2026-03-23

## Added
- [#229] add rich error info for simulation failures
- add support for `FundsWithdrawal` transactions
- add coin balance selection helpers with support for address balances and coin exclusions

## Changed
- [#209] require `Sync` bound for `IntentResolver`

[#209]: https://github.com/MystenLabs/sui-rust-sdk/pull/209
[#229]: https://github.com/MystenLabs/sui-rust-sdk/pull/229

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

[0.3.1]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-transaction-builder-0.3.1
[0.3.0]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-transaction-builder-0.3.0
[0.2.2]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-transaction-builder-0.2.2
[0.2.1]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-transaction-builder-0.2.1
[0.2.0]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-transaction-builder-0.2.0
