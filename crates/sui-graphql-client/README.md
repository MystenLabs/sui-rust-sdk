# sui-graphql-client

[![sui-graphql-client on crates.io](https://img.shields.io/crates/v/sui-graphql-client)](https://crates.io/crates/sui-graphql-client)
[![Documentation (latest release)](https://img.shields.io/badge/docs-latest-brightgreen)](https://docs.rs/sui-graphql-client)
[![Documentation (master)](https://img.shields.io/badge/docs-master-59f)](https://mystenlabs.github.io/sui-rust-sdk/sui-graphql-client/)

The Sui GraphQL client is a client for interacting with the Sui blockchain via GraphQL.
It provides a set of APIs for querying the blockchain for information such as chain identifier,
reference gas price, protocol configuration, service configuration, checkpoint, epoch,
executing transactions and more.

# Design Principles

1. **Type Safety**: The client uses the `cynic` library to generate types from the schema. This ensures that the queries are type-safe.
1. **Convenience**: The client provides a set of APIs for common queries such as chain identifier, reference gas price, protocol configuration, service configuration, checkpoint, epoch, executing transactions and more.
1. **Custom Queries**: The client provides a way to run custom queries using the `cynic` library.

# Usage

## Connecting to a GraphQL server
Instantiate a client with [`Client::new(server: &str)`] or use one of the predefined functions for different networks [`Client`].

```rust, no_run
use sui_graphql_client::Client;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {

   // Connect to the mainnet GraphQL server
   let client = Client::new_mainnet();
   let chain_id = client.chain_id().await?;
   println!("{:?}", chain_id);

   Ok(())
}
```

## Requesting gas from the faucet
The client provides an API to request gas from the faucet. The `request_and_wait` function sends a request to the faucet and waits until the transaction is confirmed. The function returns the transaction details if the request is successful.

### Example for standard devnet/testnet/local networks.
```rust, no_run
use sui_graphql_client::faucet::FaucetClient;
use sui_types::types::Address;

use anyhow::Result;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    let address = Address::from_str("SUI_ADDRESS_HERE")?;
    // Request gas from the faucet and wait until a coin is received
    // As the client is set to devnet, faucet will use the devnet faucet.
    let faucet = FaucetClient::devnet().request_and_wait(address).await?;
    if let Some(resp) = faucet {
        let coins = resp.sent;
        for coin in coins {
            println!("coin: {:?}", coin);
        }
    }

    // Request gas from the testnet faucet by explicitly setting the faucet to testnet
    let faucet_testnet = FaucetClient::testnet().request_and_wait(address).await?;
    Ok(())
}
```

### Example for custom faucet service.
Note that this [`FaucetClient`] is explicitly designed to work with two endpoints: `v1/gas`, and `v1/status`. When passing in the custom faucet URL, skip the final endpoint and only pass in the top-level url (e.g., `https://faucet.devnet.sui.io`).
```rust, no_run
use sui_graphql_client::faucet::FaucetClient;
use sui_types::types::Address;

use anyhow::Result;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    let address = Address::from_str("SUI_ADDRESS_HERE")?;
    // Request gas from the faucet and wait until a coin is received
    // As the client is set to devnet, faucet will use the devnet faucet.
    let faucet = FaucetClient::new("https://myfaucet_testnet.com").request_and_wait(address).await?;
    if let Some(resp) = faucet {
        let coins = resp.sent;
        for coin in coins {
            println!("coin: {:?}", coin);
        }
    }
    Ok(())
}
```

## Custom Queries
There are several options for running custom queries.
1) Use a GraphQL client library of your choosing.
2) Use the [cynic's web generator](https://generator.cynic-rs.dev/) that accepts as input the schema and generates the query types.
3) Use the [cynic's CLI](https://github.com/obmarg/cynic/tree/main/cynic-cli) and use the `cynic querygen` command to generate the query types.

Below is an example that uses the `cynic querygen` CLI to generate the query types from the schema and the following query:
```bash
cynic querygen --schema rpc.graphql --query custom_query.graphql
```
where `custom_query.graphql` contains the following query:

```graphql
query CustomQuery($id: UInt53) {
  epoch(id: $id) {
    referenceGasPrice
    totalGasFees
    totalCheckpoints
    totalTransactions
  }
}
```

The generated query types are defined below. Note that the `id` variable is optional (to make it mandatory change the schema to $id: Uint53! -- note the ! character which indicates a mandatory field). That means that if the `id` variable is not provided, the query will return the data for the last known epoch.
Note that instead of using `Uint53`, the scalar is mapped to `u64` in the library using `impl_scalar(u64, schema::Uint53)`, thus all references to `Uint53` in the schema are replaced with `u64` in the code below.


```rust, ignore
#[derive(cynic::QueryVariables, Debug)]
pub struct CustomQueryVariables {
    pub id: Option<u64>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "CustomQueryVariables")]
pub struct CustomQuery {
    #[arguments(id: $id)]
    pub epoch: Option<Epoch>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Epoch {
    pub epoch_id: u64,
    pub reference_gas_price: Option<BigInt>,
    pub total_gas_fees: Option<BigInt>,
    pub total_checkpoints: Option<u64>,
    pub total_transactions: Option<u64>,
}

#[derive(cynic::Scalar, Debug, Clone)]
pub struct BigInt(pub String);
```

The complete example is shown below:
```rust, ignore
use anyhow::Result;
use cynic::QueryBuilder;

use sui_graphql_client::{
    query_types::{schema, BigInt},
    Client,
};
use sui_types::types::Address;

// The data returned by the custom query.
#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Epoch")]
pub struct EpochData {
    pub epoch_id: u64,
    pub reference_gas_price: Option<BigInt>,
    pub total_gas_fees: Option<BigInt>,
    pub total_checkpoints: Option<u64>,
    pub total_transactions: Option<u64>,
}

// The variables to pass to the custom query.
// If an epoch id is passed, then the query will return the data for that epoch.
// Otherwise, the query will return the data for the last known epoch.
#[derive(cynic::QueryVariables, Debug)]
pub struct CustomVariables {
    pub id: Option<u64>,
}

// The custom query. Note that the variables need to be explicitly declared.
#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query", variables = "CustomVariables")]
pub struct CustomQuery {
    #[arguments(id: $id)]
    pub epoch: Option<EpochData>,
}

// Custom query with no variables.
#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query")]
pub struct ChainIdQuery {
    chain_identifier: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = Client::new_devnet();

    // Query the data for the last known epoch. Note that id variable is None, so last epoch data
    // will be returned.
    let operation = CustomQuery::build(CustomVariables { id: None });
    let response = client
        .run_query::<CustomQuery, CustomVariables>(&operation)
        .await;
    println!("{:?}", response);

    // Query the data for epoch 1.
    let epoch_id = 1;
    let operation = CustomQuery::build(CustomVariables { id: Some(epoch_id) });
    let response = client
        .run_query::<CustomQuery, CustomVariables>(&operation)
        .await;
    println!("{:?}", response);

    // When the query has no variables, just pass () as the type argument
    let operation = ChainIdQuery::build(());
    let response = client.run_query::<ChainIdQuery, ()>(&operation).await?;
    if let Some(chain_id) = response.data {
        println!("Chain ID: {}", chain_id.chain_identifier);
    }

    Ok(())
}
```

