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

```rust
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


```rust,ignore
#[derive(cynic::QueryVariables, Debug)]
pub struct CustomQueryVariables {
    pub id: Option<Uint53>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "CustomQueryVariables")]
pub struct CustomQuery {
    #[arguments(id: $id)]
    pub epoch: Option<Epoch>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Epoch {
    pub epoch_id: Uint53,
    pub reference_gas_price: Option<BigInt>,
    pub total_gas_fees: Option<BigInt>,
    pub total_checkpoints: Option<Uint53>,
    pub total_transactions: Option<Uint53>,
}

#[derive(cynic::Scalar, Debug, Clone)]
pub struct BigInt(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
#[cynic(graphql_type = "UInt53")]
pub struct Uint53(pub u64);
```

The complete example is shown below:
```rust
use anyhow::Result;
use cynic::QueryBuilder;

use sui_graphql_client::{
    query_types::{schema, BigInt, Uint53},
    Client,
};
use sui_types::types::Address;

// The data returned by the custom query.
#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Epoch")]
pub struct EpochData {
    pub epoch_id: Uint53,
    pub reference_gas_price: Option<BigInt>,
    pub total_gas_fees: Option<BigInt>,
    pub total_checkpoints: Option<Uint53>,
    pub total_transactions: Option<Uint53>,
}

// The variables to pass to the custom query.
// If an epoch id is passed, then the query will return the data for that epoch.
// Otherwise, the query will return the data for the last known epoch.
#[derive(cynic::QueryVariables, Debug)]
pub struct CustomVariables {
    pub id: Option<Uint53>,
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
    let epoch_id = Uint53(1);
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

