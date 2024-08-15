use sui_graphql_client::client::SuiClient;

#[tokio::main]
async fn main() {
    let client = SuiClient::default();
    let total_checkpoints = client.read_api().epoch_total_checkpoints(None).await;
    let epoch_summary = client.read_api().epoch_total_transaction_blocks(None).await;
    println!("{:?}", total_checkpoints);
    println!("{:?}", epoch_summary);
    println!("Hello, world!");
}
