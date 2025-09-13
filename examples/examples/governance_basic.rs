// Governance-style GraphQL example.
// Run: cargo run --example governance_basic
use sui_graphql_client::{Client, PaginationFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new_testnet();
    let validators = client
        .active_validators(
            None,
            PaginationFilter {
                limit: Some(5),
                ..Default::default()
            },
        )
        .await?;
    println!("Active validators ({} shown):", validators.data().len());
    for v in validators.data() {
        println!(" - {}", v.name.as_deref().unwrap_or("<no-name>"));
    }
    println!("Chain ID: {}", client.chain_id().await?);
    if let Some(rgp) = client.reference_gas_price(None).await? {
        println!("Reference gas price: {rgp}");
    }
    if let Some(cfg) = client.protocol_config(None).await? {
        println!("Protocol version: {}", cfg.protocol_version);
    }
    if let Some(total) = client.total_transaction_blocks().await? {
        println!("Total tx blocks: {total}");
    }
    Ok(())
}
