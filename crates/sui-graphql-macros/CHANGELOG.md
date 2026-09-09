# [0.4.0] - 2026-09-09

## Added
- [`05ee2e62`] add `#[field(flatten)]` to the `Response` derive for fields
  that receive the complete response value, so several projections can read
  different fields from the same response root; the derive now also
  generates a borrowed `extract(&serde_json::Value)` method alongside the
  owned `from_value`
- [`97fd47e7`] allow `graphql_query!` to compose a document from several
  comma-separated sources, either inline string literals or `@"path"` files
  resolved relative to the invoking Rust source file, so operations and
  shared fragments can live apart

## Changed
- [`838bea4f`] parse and validate GraphQL with `bluejay` instead of
  `apollo-compiler`, roughly halving the macro's dependency tree; the
  wording of diagnostics for invalid queries changed
- [#288] update the embedded Sui GraphQL schema with
  `ForwardingAddressRegistryCreateTransaction`

## Fixed
- [`2726dbb1`] a `Response` field named `value` no longer shadows the
  response input, so fields declared after it extract from the response
  root instead of from the already extracted value

[#288]: https://github.com/MystenLabs/sui-rust-sdk/pull/288
[`05ee2e62`]: https://github.com/mystenlabs/sui-rust-sdk/commit/05ee2e62
[`97fd47e7`]: https://github.com/mystenlabs/sui-rust-sdk/commit/97fd47e7
[`838bea4f`]: https://github.com/mystenlabs/sui-rust-sdk/commit/838bea4f
[`2726dbb1`]: https://github.com/mystenlabs/sui-rust-sdk/commit/2726dbb1

# [0.3.1] - 2026-07-16

## Added
- [#245] `graphql_query!` function-style macro that validates GraphQL queries
  and mutations against the embedded Sui schema at compile time

## Changed
- [#271] updated the embedded Sui GraphQL schema; among other changes this
  removes the `SafeMode`, `StakeSubsidy`, `StorageFund`, `SystemParameters`,
  and `ValidatorCredentials` types

[#245]: https://github.com/MystenLabs/sui-rust-sdk/pull/245
[#271]: https://github.com/MystenLabs/sui-rust-sdk/pull/271

# [0.3.0] - 2026-03-23

Initial published release.

## Added
- [#194] `QueryResponse` derive macro for field extraction from GraphQL responses
- [#195] array extraction support to `QueryResponse` macro
- [#196] compile-time schema validation for `QueryResponse`
- [#210] custom schema support
- [#211] type-driven array extraction
- [#215] unified path parsing via `ParsedPath`
- [#220] per-segment null handling in field paths
- [#224] crate-level documentation and README
- [#230] consistent naming for `DynamicField` methods

[#194]: https://github.com/MystenLabs/sui-rust-sdk/pull/194
[#195]: https://github.com/MystenLabs/sui-rust-sdk/pull/195
[#196]: https://github.com/MystenLabs/sui-rust-sdk/pull/196
[#210]: https://github.com/MystenLabs/sui-rust-sdk/pull/210
[#211]: https://github.com/MystenLabs/sui-rust-sdk/pull/211
[#215]: https://github.com/MystenLabs/sui-rust-sdk/pull/215
[#220]: https://github.com/MystenLabs/sui-rust-sdk/pull/220
[#224]: https://github.com/MystenLabs/sui-rust-sdk/pull/224
[#230]: https://github.com/MystenLabs/sui-rust-sdk/pull/230

[0.4.0]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-graphql-macros-0.4.0
[0.3.1]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-graphql-macros-0.3.1
[0.3.0]: https://github.com/mystenlabs/sui-rust-sdk/releases/tag/sui-graphql-macros-0.3.0
