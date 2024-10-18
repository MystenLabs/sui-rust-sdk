use anyhow::anyhow;
use base64ct::Encoding;
use std::str::FromStr;
use sui_crypto::ed25519::Ed25519PrivateKey;
use sui_crypto::SuiSigner;
use sui_graphql_client::Client;
use sui_transaction_builder::Object;
use sui_transaction_builder::Serialized;
use sui_transaction_builder::TransactionBuilder;
use sui_types::types::Address;
use sui_types::types::ObjectDigest;
use sui_types::types::ObjectId;
use sui_types::types::UserSignature;

async fn example_without_literal_inference() -> Result<(), anyhow::Error> {
    let client = Client::new_localhost();
    //
    let mut tx = TransactionBuilder::new();
    let coin_obj_id = "0x19406ea4d9609cd9422b85e6bf2486908f790b778c757aff805241f3f609f9b4";
    let coin_digest = "7opR9rFUYivSTqoJHvFb9p6p54THyHTatMG6id4JKZR9";
    let coin_version = 2;
    //
    let coin = tx.input(Object::owned(
        ObjectId::from_str(coin_obj_id).unwrap(),
        coin_version,
        ObjectDigest::from_str(coin_digest).map_err(|_| anyhow!("Invalid object digest"))?,
    ));

    let recipient = tx.input(Serialized(
        &Address::from_str("0xeeaeb20e7b3a9cfce768a2ddf69503d8ae8d1628a26c2d74a435501c929d3f69")
            .map_err(|_| anyhow!("Invalid address"))?,
    ));

    tx.transfer_objects(vec![coin], recipient);
    tx.set_gas_budget(500000000);
    tx.set_gas_price(1000);
    tx.set_gas(vec![Object::owned(
        ObjectId::from_str("0xd8792bce2743e002673752902c0e7348dfffd78638cb5367b0b85857bceb9821")
            .unwrap(),
        2,
        ObjectDigest::from_str("2ZigdvsZn5BMeszscPQZq9z8ebnS2FpmAuRbAi9ednCk")
            .map_err(|_| anyhow!("Invalid object digest"))?,
    )]);
    tx.set_sender(
        Address::from_str("0xc574ea804d9c1a27c886312e96c0e2c9cfd71923ebaeb3000d04b5e65fca2793")
            .unwrap(),
    );

    let tx = tx.finish()?;
    //
    // let dry_run = client.dry_run_tx(&tx, None).await?;
    // println!("{:?}", dry_run);

    // let pk = base64ct::Base64::decode_vec("ALq6tKX5UqW5AyEqMj5Xq/zOC5tlKl/plh0afwooHZAn").unwrap();
    // let pk = base64ct::Base64::decode_vec("APE5yOYt6zlGz+C4sV9S09A3bORT6h9vumk4zP7+7H1a").unwrap();
    let pk = base64ct::Base64::decode_vec("AAjgPs/zbxDsObju6Tp2/8W5lNWP/sjUDsVxR1vgdmyT").unwrap();
    let pk = Ed25519PrivateKey::new(pk[1..].try_into().expect("slice has wrong length"));

    println!("{:?}", pk.public_key());
    println!("{:?}", pk.public_key().to_address());
    println!(
        "{:?}",
        base64ct::Base64::encode_string(pk.public_key().as_bytes())
    );
    let t = pk.sign_transaction(&tx);

    println!("{:?}", t);

    //
    // // println!("{:?}", t);
    // let sig = "AGPG9CToY0r+3Uss5r2CVeHjD9rTZIvOhWLCqKe0Nj4JAfLZCnmJCGFgj+kC2/JHW4jRIsTfVUICbvRXyE3YtQGb4j7voyHkHCbVWoY0M/QxU8+gFDigeOxJIVqeaxKMSQ==";
    // let sig = UserSignature::from_base64(&sig).unwrap();
    //
    let effects = client.execute_tx(vec![t.unwrap()], &tx).await?;
    println!("{:?}", effects);
    // let split = tx.split_coins(
    //     coin,
    //    vec![tx.input(Serialized(&500u64)), tx.input(Serialized(&500u64))],
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
