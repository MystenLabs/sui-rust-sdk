// Basic example using sui-graphql-client to fetch chain id and first few coins.
// Run: cargo run --example graphql_basic
use sui_graphql_client::{Client, PaginationFilter};
use sui_sdk_types::Address;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new_testnet();
    let chain_id = client.chain_id().await?;
    println!("Chain ID: {chain_id}");
    let address: Address = "0x1".parse()?;
    let balance = client.balance(address, None).await?;
    println!("Balance (SUI) for 0x1: {:?}", balance);
    let coins = client
        .coins(
            address,
            None,
            PaginationFilter {
                limit: Some(5),
                ..Default::default()
            },
        )
        .await?;
    println!("Coins page size: {}", coins.data().len());
    if let Some(cp) = client.checkpoint(None, None).await? {
        println!(
            "Latest checkpoint seq: {} total tx: {}",
            cp.sequence_number, cp.network_total_transactions
        );
    }
    Ok(())
}
