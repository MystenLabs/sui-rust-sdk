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

[0.0.8]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-rpc-0.0.8
[0.0.7]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-rpc-0.0.7
[0.0.6]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-rpc-0.0.6
