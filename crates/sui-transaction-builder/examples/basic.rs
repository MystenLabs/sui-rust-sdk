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

    let mut tx = TransactionBuilder::new();
    let coin_obj_id = "0x530b2f240cfa1e31d41cb1a4ce57b73084c767546feb1d8823809180114ed8b9";
    let coin_digest = "6Tg9GrJnKSg6XXQv7eE2usMaN7EaaxbVvv34w5J4oBKo";
    let coin_version = 2;

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
        ObjectId::from_str("0x2e3a2b36a01cd6ad6b39788fc2bcbe9944ac482bccba1e9c6adea29a3c406f76")
            .unwrap(),
        2,
        ObjectDigest::from_str("6A1VyFnpKKG28oaM9G4gwkkfh8DSAByxSBTQJvoyTMhd")
            .map_err(|_| anyhow!("Invalid object digest"))?,
    )]);
    tx.set_sender(
        Address::from_str("0x462ee1c227fc6365f842e2f4c9b7f66bbead607b128b68a7f7695cf2224c5168")
            .unwrap(),
    );

    let tx = tx.finish()?;

    let dry_run = client.dry_run_tx(&tx, None).await?;
    println!("{:?}", dry_run);

    let pk = base64ct::Base64::decode_vec("ALq6tKX5UqW5AyEqMj5Xq/zOC5tlKl/plh0afwooHZAn").unwrap();
    let pk = Ed25519PrivateKey::new(pk[1..].try_into().expect("slice has wrong length"));

    // let t = pk.sign_transaction(&tx);
    // println!("{:?}", t);
    //
    // // println!("{:?}", t);
    // let sig = "AGPG9CToY0r+3Uss5r2CVeHjD9rTZIvOhWLCqKe0Nj4JAfLZCnmJCGFgj+kC2/JHW4jRIsTfVUICbvRXyE3YtQGb4j7voyHkHCbVWoY0M/QxU8+gFDigeOxJIVqeaxKMSQ==";
    // let sig = UserSignature::from_base64(&sig).unwrap();
    //
    // let effects = client.execute_tx(vec![sig], &tx).await?;
    // println!("{:?}", effects);
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
