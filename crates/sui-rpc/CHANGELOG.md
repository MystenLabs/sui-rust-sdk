# [0.3.2] - 2026-07-16

## Added
- [#256] [#264] [#265] [#278] add filtered `ListCheckpoints`,
  `ListTransactions`, and `ListEvents` RPCs to the v2 `LedgerService`,
  filtered `SubscribeTransactions` and `SubscribeEvents` RPCs to the v2
  `SubscriptionService`, and a `filter` field on
  `SubscribeCheckpointsRequest`
- [#270] [#275] [`f3673da1`] add transaction and event filter protos with
  ergonomic builders in the v2 `filter` module
- [#263] add support for a lossless `configs` field on `ProtocolConfig`
- [`3ef25b50`] add `Client::with_response_headers_timeout`, a client-wide
  timeout enforced until response headers arrive (effectively a total
  deadline for unary calls)
- [`57343980`] [`4a8d5e66`] [`d2a753b0`] [`4bd29a1e`] add a `light_client`
  module behind the `unstable` feature: a `LightClient` facade with an
  `EpochCache` and a BLS-verifying committee ratchet driver, whose
  `prove_object_at_checkpoint` authenticates object inclusion or
  non-inclusion at a checkpoint against a trusted committee
- [`99ed1329`] [`b60bc5cc`] [`b1de5e3e`] add the v2alpha `ProofService`
  (`GetCheckpointObjectProof`) and a `Client::proof_client` accessor
  behind the `unstable` feature
- [`1f90fea3`] [#266] add `AuthenticatedEventsClient`, a streaming
  verifier that authenticates event streams against the on-chain
  `EventStreamHead`, behind the `unstable` feature
- [`69bbfacd`] [`bd694c52`] [`e7d374f1`] [`0db0303d`] add `RatchetConfig`
  (parallel end-of-epoch fetches, transient-failure retry, and a
  `max_ratchet_gap` cap) and `LightClient::with_archive` for
  archive-first historical reads, behind the `unstable` feature

## Changed
- [`668c7a3c`] [`ab33fca8`] harden the shared HTTP/2 connection against
  flow-control starvation: `Client::new` now sets explicit stream and
  connection receive windows (overridable with
  `Client::with_initial_stream_window_size` and
  `Client::with_initial_connection_window_size`), and a default-on
  idle-body watchdog (configurable via `Client::with_body_idle_timeout`
  or a per-request `BodyIdleTimeout` extension) reaps response streams
  that stop making progress
- [`e3292e9f`] floor hyper at 1.10 to exclude h2 versions with
  flow-control accounting bugs that can wedge a multiplexed connection
- [#286] [`b675bf93`] [`acd4e828`] shrink `Client` by moving its shared
  configuration behind an `Arc`, and box the large intermediate futures
  in the staking and execute-and-wait helpers

## Fixed
- [`fb3d25a2`] enforce the deadline set with `tonic::Request::set_timeout`
  across the whole response body, so a per-call deadline now bounds the
  entire call instead of only the response-headers phase
- [`9ee937f9`] poll the checkpoint subscription while executing in
  `execute_transaction_and_wait_for_checkpoint`, so long executions no
  longer starve the subscription and fail with a spurious
  `CheckpointStreamError`
- [`0f16029e`] follow empty pagination pages in the `list_owned_objects`,
  `list_dynamic_fields`, `list_balances`, and `list_package_versions`
  streams instead of silently dropping the remaining pages
- [#283] resolve staking pools in pages in `Client::calculate_rewards` so
  a single call cannot build a transaction too large to run
- [#287] handle duplicate submissions in
  `execute_transaction_and_wait_for_checkpoint`: a ledger probe now races
  the execution RPC so an already committed transaction is returned
  without waiting for execution to finish, and an execution error is
  downgraded to success when the ledger shows the transaction in a
  checkpoint

[#256]: https://github.com/MystenLabs/sui-rust-sdk/pull/256
[#263]: https://github.com/MystenLabs/sui-rust-sdk/pull/263
[#264]: https://github.com/MystenLabs/sui-rust-sdk/pull/264
[#265]: https://github.com/MystenLabs/sui-rust-sdk/pull/265
[#266]: https://github.com/MystenLabs/sui-rust-sdk/pull/266
[#270]: https://github.com/MystenLabs/sui-rust-sdk/pull/270
[#275]: https://github.com/MystenLabs/sui-rust-sdk/pull/275
[#278]: https://github.com/MystenLabs/sui-rust-sdk/pull/278
[#283]: https://github.com/MystenLabs/sui-rust-sdk/pull/283
[#286]: https://github.com/MystenLabs/sui-rust-sdk/pull/286
[#287]: https://github.com/MystenLabs/sui-rust-sdk/pull/287

[`f3673da1`]: https://github.com/mystenlabs/sui-rust-sdk/commit/f3673da1
[`3ef25b50`]: https://github.com/mystenlabs/sui-rust-sdk/commit/3ef25b50
[`57343980`]: https://github.com/mystenlabs/sui-rust-sdk/commit/57343980
[`4a8d5e66`]: https://github.com/mystenlabs/sui-rust-sdk/commit/4a8d5e66
[`d2a753b0`]: https://github.com/mystenlabs/sui-rust-sdk/commit/d2a753b0
[`4bd29a1e`]: https://github.com/mystenlabs/sui-rust-sdk/commit/4bd29a1e
[`99ed1329`]: https://github.com/mystenlabs/sui-rust-sdk/commit/99ed1329
[`b60bc5cc`]: https://github.com/mystenlabs/sui-rust-sdk/commit/b60bc5cc
[`b1de5e3e`]: https://github.com/mystenlabs/sui-rust-sdk/commit/b1de5e3e
[`1f90fea3`]: https://github.com/mystenlabs/sui-rust-sdk/commit/1f90fea3
[`69bbfacd`]: https://github.com/mystenlabs/sui-rust-sdk/commit/69bbfacd
[`bd694c52`]: https://github.com/mystenlabs/sui-rust-sdk/commit/bd694c52
[`e7d374f1`]: https://github.com/mystenlabs/sui-rust-sdk/commit/e7d374f1
[`0db0303d`]: https://github.com/mystenlabs/sui-rust-sdk/commit/0db0303d
[`668c7a3c`]: https://github.com/mystenlabs/sui-rust-sdk/commit/668c7a3c
[`ab33fca8`]: https://github.com/mystenlabs/sui-rust-sdk/commit/ab33fca8
[`e3292e9f`]: https://github.com/mystenlabs/sui-rust-sdk/commit/e3292e9f
[`b675bf93`]: https://github.com/mystenlabs/sui-rust-sdk/commit/b675bf93
[`acd4e828`]: https://github.com/mystenlabs/sui-rust-sdk/commit/acd4e828
[`fb3d25a2`]: https://github.com/mystenlabs/sui-rust-sdk/commit/fb3d25a2
[`9ee937f9`]: https://github.com/mystenlabs/sui-rust-sdk/commit/9ee937f9
[`0f16029e`]: https://github.com/mystenlabs/sui-rust-sdk/commit/0f16029e

# [0.3.1] - 2026-04-13

## Added
- [#233] make `Client::calculate_rewards` and
  `Client::get_validator_address_by_pool_id` public
- [#235] allow constructing a `Client` from a configured tonic `Endpoint`
- [#237] add support for user-provided request middleware via
  `Client::request_layer`, which wraps all outbound RPC requests with an
  arbitrary `tower::Layer`

## Changed
- [#236] update the default endpoint configuration

[#233]: https://github.com/MystenLabs/sui-rust-sdk/pull/233
[#235]: https://github.com/MystenLabs/sui-rust-sdk/pull/235
[#236]: https://github.com/MystenLabs/sui-rust-sdk/pull/236
[#237]: https://github.com/MystenLabs/sui-rust-sdk/pull/237

# [0.3.0] - 2026-03-23

## Added
- [#216] add support for `Object.display` and `SimulateTransactionResponse.suggested_gas_price`
- [#231] add proto support for `AccumulatorValue::EventDigest` and `AccumulatorValue::IntegerTuple`
  for authenticated event streams
- add `Unimplemented*` default stubs for all generated gRPC service traits

## Fixed
- [#228] box `FieldViolation` in `TryFromProtoError` to reduce stack size
- [#212] handle the case where a transaction has already been executed
- support partial errors in `Object.display`

[#212]: https://github.com/MystenLabs/sui-rust-sdk/pull/212
[#216]: https://github.com/MystenLabs/sui-rust-sdk/pull/216
[#228]: https://github.com/MystenLabs/sui-rust-sdk/pull/228
[#231]: https://github.com/MystenLabs/sui-rust-sdk/pull/231

# [0.2.2] - 2026-01-20

## Added
- [#202] add support for TransactionKind::ProgrammableSystemTransaction
- [#204] add support for EndOfEpochTransactionKind::WriteAccumulatorStorageCost

[#202]: https://github.com/MystenLabs/sui-rust-sdk/pull/202
[#204]: https://github.com/MystenLabs/sui-rust-sdk/pull/204

# [0.2.1] - 2026-01-07

## Added
- [#185] add `address_balance` and `coin_balance` fields to Balance message

[#185]: https://github.com/MystenLabs/sui-rust-sdk/pull/185

# [0.2.0] - 2026-01-05

## Added
- Added support for address balances [#179]
- Added support for address aliases [#177]
- Added support for CheckpointContents V2 [#180]

[#177]: https://github.com/MystenLabs/sui-rust-sdk/pull/177
[#179]: https://github.com/MystenLabs/sui-rust-sdk/pull/179
[#180]: https://github.com/MystenLabs/sui-rust-sdk/pull/180

# [0.1.1] - 2025-12-11

## Added
- Added new move vm adapter error variants

# [0.1.0] - 2025-11-07

## Changed
- Updated to rust 2024 edition [#171]
- Updated to tonic/prost 0.14 [#168]
- Removed `Into` requirement for setters of primitive types [`ec1547f1`]
- Changed interceptor to allow for adding headers to all requests [`31ae830f`]
- Removed v2beta2 protos and client

[#171]: https://github.com/MystenLabs/sui-rust-sdk/pull/171
[#168]: https://github.com/MystenLabs/sui-rust-sdk/pull/168
[`ec1547f1`]: https://github.com/mystenlabs/sui-rust-sdk/commit/ec1547f1
[`31ae830f`]: https://github.com/mystenlabs/sui-rust-sdk/commit/31ae830f

# [0.0.8] - 2025-10-03

## Added
- Support for field path builders for proto messages
- Support for field accessors and builder methods for proto messages
- Add `Client::execute_transaction_and_wait_for_checkpoint` method
- Add support for `sui.rpc.v2` proto package which will be stabilized in sui v1.58
- Add `Stream` based methods on `Client` for the various `List*` service apis.
- Add coin selection methods on `Client`
- Move the `FaucetClient` here from `sui-graphql-client`

# [0.0.7] - 2025-08-29

## Added

- Added urls for the foundation provided, public-good rpc infrastructure ([`84f088d5`])
- Added NameService protos ([`0a155205`])
- Added helper for fetching staking information ([`8bc0ff52`])

## Changed

- Use BTreeMap for all map fields ([`9a79aa5b`])
- Marked all protos with non_exhaustive ([`5191506d`])
- Renamed shared -> consensus ([`cab42748`])
- Selected tls-ring backend ([`2b37ade0`])
- Updated vendored proto with CoinRegistry changes ([`72bef2c8`])
- Update to use version 0.0.7 of `sui-sdk-types`

[`84f088d5`]: https://github.com/mystenlabs/sui-rust-sdk/commit/84f088d5
[`9a79aa5b`]: https://github.com/mystenlabs/sui-rust-sdk/commit/9a79aa5b
[`5191506d`]: https://github.com/mystenlabs/sui-rust-sdk/commit/5191506d
[`0a155205`]: https://github.com/mystenlabs/sui-rust-sdk/commit/0a155205
[`8bc0ff52`]: https://github.com/mystenlabs/sui-rust-sdk/commit/8bc0ff52
[`cab42748`]: https://github.com/mystenlabs/sui-rust-sdk/commit/cab42748
[`2b37ade0`]: https://github.com/mystenlabs/sui-rust-sdk/commit/2b37ade0
[`72bef2c8`]: https://github.com/mystenlabs/sui-rust-sdk/commit/72bef2c8

# [0.0.6] - 2025-07-16

Initial release

[0.3.2]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-rpc-0.3.2
[0.3.1]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-rpc-0.3.1
[0.3.0]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-rpc-0.3.0
[0.2.2]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-rpc-0.2.2
[0.2.1]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-rpc-0.2.1
[0.2.0]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-rpc-0.2.0
[0.1.1]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-rpc-0.1.1
[0.1.0]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-rpc-0.1.0
[0.0.8]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-rpc-0.0.8
[0.0.7]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-rpc-0.0.7
[0.0.6]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-rpc-0.0.6
