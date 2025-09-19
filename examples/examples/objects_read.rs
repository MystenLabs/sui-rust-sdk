// Object read example via GraphQL.
// Run: cargo run --example objects_read
use sui_graphql_client::{Client, PaginationFilter};
use sui_sdk_types::Address;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new_testnet();
    let address: Address = "0x1".parse()?;
    use sui_graphql_client::query_types::ObjectFilter;
    let page = client
        .objects(
            Some(ObjectFilter {
                type_: None,
                owner: Some(address),
                object_ids: None,
            }),
            PaginationFilter {
                limit: Some(5),
                ..Default::default()
            },
        )
        .await?;
    println!("Owned objects count page: {}", page.data().len());
    if let Some(first) = page.data().first() {
        println!("First object (debug): {:?}", first);
    }
    Ok(())
}
