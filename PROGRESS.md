# Sui GraphQL Rust SDK - Development Progress

## Project Overview

Building a Rust SDK for Sui's GraphQL API in the `sui-rust-sdk` repository. The SDK provides type-safe GraphQL queries with automatic BCS deserialization using a custom `Response` derive macro.

## Repository Structure

```
sui-rust-sdk/
├── crates/
│   ├── sui-graphql/           # Main GraphQL client library
│   │   ├── src/
│   │   │   ├── client/        # API methods (coins, chain, objects, etc.)
│   │   │   ├── bcs.rs         # Bcs<T> wrapper with Deserialize impl
│   │   │   ├── scalars.rs     # GraphQL scalars (BigInt, DateTime, etc.)
│   │   │   ├── error.rs       # Error types
│   │   │   ├── pagination.rs  # Pagination utilities
│   │   │   └── response.rs    # Response handling
│   │   └── examples/          # (stashed) execute_transaction example
│   └── sui-graphql-macros/    # Response derive macro
├── rust-toolchain.toml        # Local only, channel = "1.90"
└── PROGRESS.md                # This file
```

## Stacked PR Workflow

Branches are stacked and need to be rebased in order:

| Order | Branch Name | Description | Status |
|-------|-------------|-------------|--------|
| 1st | tpham-graphql-sdk-1-base | Base client setup | Done |
| 2nd | tpham-graphql-sdk-2-response-macro | Response derive macro | Done |
| 3rd | tpham-graphql-sdk-3-path-extraction | Path extraction in macro | Done |
| 4th | tpham-graphql-sdk-4-validation-tests | Schema validation | Done |
| 5th | tpham-graphql-pagination | Pagination support | Done |
| 6th | tpham-graphql-pagination-objects | Objects API with pagination | Done |
| 7th | tpham-graphql-coins | Coins/balance API | Done |
| 8th | tpham-graphql-chain-info | Chain info (epoch, protocol) | Done |
| 9th | tpham-graphql-transactions | Transaction fetching | Done |
| 10th | tpham-graphql-checkpoints | Checkpoint fetching | Done |
| 11th | tpham-graphql-dynamic-field | Dynamic fields API | Done |
| 12th | tpham-graphql-execution | Transaction execution | Done (current) |

## Key Patterns Established

### 1. Bcs<T> Wrapper (crates/sui-graphql/src/bcs.rs)

Automatically decodes Base64+BCS in GraphQL responses:

```rust
pub struct Bcs<T>(pub T);

impl<'de, T: DeserializeOwned> Deserialize<'de> for Bcs<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let b64 = <Cow<'_, str>>::deserialize(deserializer)?;
        let bytes = Base64::decode_vec(&b64).map_err(serde::de::Error::custom)?;
        let value = bcs::from_bytes(&bytes).map_err(serde::de::Error::custom)?;
        Ok(Bcs(value))
    }
}
```

Usage in Response structs:
```rust
#[derive(Response)]
struct Response {
    #[field(path = "transaction.transactionBcs")]
    transaction_bcs: Option<Bcs<Transaction>>,  // Automatically decoded
}

// Access inner value with .0
let transaction = data.transaction_bcs.map(|bcs| bcs.0);
```

### 2. BigInt Scalar (crates/sui-graphql/src/scalars.rs)

Parses string-encoded u64 values from GraphQL:

```rust
pub struct BigInt(pub u64);

impl<'de> Deserialize<'de> for BigInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <Cow<'_, str>>::deserialize(deserializer)?;
        let value = s.parse().map_err(serde::de::Error::custom)?;
        Ok(BigInt(value))
    }
}
```

Usage:
```rust
#[field(path = "address.balance.totalBalance")]
total_balance: Option<BigInt>,

// Access: total_balance.map(|b| b.0)
```

### 3. StructTag for coin_type

`StructTag` from `sui_sdk_types` implements `Deserialize` with `DisplayFromStr`, so it can be used directly:

```rust
#[field(path = "address.balance.coinType.repr")]
coin_type: Option<StructTag>,  // Automatically parsed from string
```

### 4. Response Macro Pattern

```rust
#[derive(Response)]
struct Response {
    #[field(path = "some.nested.path")]
    field_name: Option<Type>,
}

// For mutations:
#[derive(Response)]
#[response(mutation)]
struct Response { ... }
```

### 5. Error Handling

- Deserialization errors are wrapped in `Error::Request(reqwest::Error)` with `is_decode()` returning true
- Use `Error::MissingData(&'static str)` for required fields that are missing

## API Methods Implemented

### Client (crates/sui-graphql/src/client/mod.rs)
- `Client::new(url)` - Create client
- `Client::query(query, variables)` - Raw GraphQL query

### Chain Info (chain.rs)
- `chain_identifier()` -> `Option<Digest>`
- `protocol_version()` -> `u64`
- `epoch(epoch_id: Option<u64>)` -> `Option<Epoch>`

### Coins (coins.rs)
- `get_balance(owner, coin_type)` -> `Option<Balance>`
- `list_balances(owner)` -> `Stream<Balance>`

### Objects (objects.rs)
- `get_object(object_id)` -> `Option<Object>`
- `list_objects(owner)` -> `Stream<Object>`

### Transactions (transactions.rs)
- `get_transaction(digest)` -> `Option<TransactionResponse>`

### Checkpoints (checkpoints.rs)
- `get_checkpoint(sequence_number)` -> `Option<CheckpointResponse>`

### Dynamic Fields (dynamic_fields.rs)
- `get_dynamic_fields(parent_id)` -> `Stream<DynamicField>`

### Execution (execution.rs)
- `execute_transaction(tx_bytes, signatures)` -> `ExecutionResult`

## Stashed Examples

```bash
git stash list
# stash@{0}: On tpham-graphql-execution: stash execute_transaction example
# stash@{1}: On tpham-graphql-pagination-objects: stash test_transaction example
```

To restore execute_transaction example:
```bash
git stash show -p stash@{0}  # View contents
# Then manually recreate since it was stashed as a deletion
```

The execute_transaction example demonstrates:
1. Loading private key from env (suiprivkey1... format)
2. Deriving address from Ed25519 key
3. Getting gas coin via list_objects
4. Building PTB (split coins + transfer)
5. Signing with sui_crypto
6. Executing via GraphQL

## Testing

Run all tests:
```bash
cargo test -p sui-graphql
```

Current test count: 37 tests passing

Tests use `wiremock` for mocking GraphQL responses.

## Recent Changes (This Session)

1. **Rebased branches 8-12** with new `Bcs<T>` and `BigInt` patterns

2. **Updated files per branch:**
   - chain.rs: `BigInt` for `reference_gas_price`
   - transactions.rs: `Bcs<T>` for transaction, effects, events
   - checkpoints.rs: `Bcs<T>` for summary and contents
   - execution.rs: `Bcs<T>` for effects

3. **Verified execute_transaction works on testnet**
   - Successfully executed a split+transfer transaction
   - Transaction digest: `7TX9Kh4ri3YrLBXqyD8Ehp598qBxBohAcoThoEu7z4P8`

## Current State

- **Branch**: `tpham-graphql-execution` (12th in stack)
- **Latest commit**: `937be36` - "Use Bcs<T> directly in Response structs for execution and transactions"
- **All 37 tests passing**
- **Local file**: `rust-toolchain.toml` (untracked, for local dev only)

## Commands Reference

```bash
# Run tests
cargo test -p sui-graphql

# Build example
cargo build --example execute_transaction -p sui-graphql

# Run example (needs PRIVATE_KEY env var)
PRIVATE_KEY=suiprivkey1... cargo run --example execute_transaction -p sui-graphql

# Get testnet private key
sui client active-address
sui keytool export --key-identity <address>

# Rebase workflow
git checkout <branch>
git rebase <previous-branch>
# Resolve conflicts, then:
git add .
git rebase --continue
```

## TL Feedback Applied

1. Use `StructTag` directly for `coin_type` (has `Deserialize` via `DisplayFromStr`)
2. Create `BigInt` struct with `Deserialize` impl for string-encoded u64
3. Use `Bcs<T>` directly in Response structs instead of `String` + manual decoding
4. Consistent error handling style: `map_err(serde::de::Error::custom)`
