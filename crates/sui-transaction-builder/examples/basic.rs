use anyhow::anyhow;
use std::str::FromStr;
use sui_graphql_client::Client;
use sui_transaction_builder::Object;
use sui_transaction_builder::Serialized;
use sui_transaction_builder::Transaction;
use sui_types::types::Address;
use sui_types::types::ObjectId;

async fn example_without_literal_inference() -> Result<(), anyhow::Error> {
    let client = Client::new_testnet();
    // let wallet = /* ... */; // TBD how one gains access to a wallet.

    let mut tx = Transaction::new();

    let coin = tx.input(Object::by_id(
        ObjectId::from_str("0x8e78225d72b1d7b1f63e5e9f88f09b12ca66c84e2fc8b91fc10f6a0c51230615")
            .map_err(|_| anyhow!("Invalid object ID"))?,
    ));

    let recipient = tx.input(Serialized(
        &Address::from_str("0xb67959920a1ea3e54ec15a1423fa0f3e5019c3b8e254df8b2a27dfab49049917")
            .map_err(|_| anyhow!("Invalid address"))?,
    ));

    println!("{:?}", tx);

    // let split = tx.split_coins(
    //     coin,
    //     vec![tx.input(Serialized(&500u64)), tx.input(Serialized(&500u64))],
    // );
    //
    // let merge = tx.merge_coins(split.nested(0), vec![split.nested(1)]);
    // tx.transfer_objects(vec![merge], recipient);
    //
    // let effects = wallet.sign_and_execute(&client, tx).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    example_without_literal_inference().await.unwrap();
}
