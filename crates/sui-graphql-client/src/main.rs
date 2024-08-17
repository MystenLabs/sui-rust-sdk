use std::str::FromStr;

use base64ct::Encoding;
use sui_graphql_client::graphql_types::ObjectFilter;
use sui_graphql_client::SuiClient;
use sui_types::types::Address;
use sui_types::types::Event;
use sui_types::types::SignedTransaction;
#[tokio::main]
async fn main() {
    let mut client = SuiClient::default();
    // client.set_mainnet();
    client.set_devnet();
    client.set_version(Some("beta"));
    let total_checkpoints = client.epoch_total_checkpoints(None).await;
    let epoch_summary = client.epoch_total_transaction_blocks(None).await;
    println!("Total Checkpoints {:?}", total_checkpoints);
    println!("Epoch summary {:?}", epoch_summary);

    // let tx = client
    // .transaction("6u6UHNpBX1qiesAL3fhhGzCfSmCzfV3VyrgJXkEcNAAM")
    // .await;

    // println!("{:?}", tx);
    //
    let coin_metadata = client.coin_metadata("0x2::sui::SUI").await;
    println!("Coin metadata {:?}", coin_metadata);

    // let objects = client.objects(None, None, None, None, None).await;
    // println!("{:?}", objects);

    let object = client
        .object(
            Address::from_str("0x003dc99ca9f8bb1c4351136fa6e941774340572e6478927270bdcd42b7efbdcc")
                .unwrap(),
            // Address::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
            //     .unwrap(),
            None,
        )
        .await;

    println!("{:?}", object);
    let filter = ObjectFilter {
        type_: None,
        owner: Some(
            Address::from_str("0x9eea5b53d1e30e07555b5e67619bfd649df752585aa0828dc3a73ed46816a8a4")
                .unwrap()
                .into(),
        ),
        object_ids: None,
        object_keys: None,
    };
    let owned_objects = client.objects(None, None, Some(filter), None, None).await;
    println!("{:?}", owned_objects);

    // let events = client.events(None, None, None, None, None).await;

    // println!("{:?}", events);

    // let bcs = "eQTgUViGyHbDyGRJEvfDAdJLYoWn93N18H+mnnJihrBOSGVsbG8sIHdvb29vb29vb29vb29vb29vb29vb29vb29vb29vb29vb29vb29vb29vb29vb29vb29vb29vb29vb29vb29vb29vb29ybGQhAQIAAwAAAAQAAAAAAAAABQAAAAAAAAAAAAAAAAAAAAYAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAwcICQ==";
    // let bcs = base64ct::Base64::decode_vec(&bcs).unwrap();
    // println!("{:?}", bcs);
    // let event: Event = bcs::from_bytes(&bcs).unwrap();
    // let bcs = "AQAAAAAAAQEBkAwKIXjgdBxrTtnPV0Kg/imCwo3W6vK+402SUxjvG3wEAAAAAAAAAAEBABzcym2+gR0xpC9SRrR4GlJn5UnXy9/92SHFq+YQ/jf+B2NvdW50ZXIJaW5jcmVtZW50AAEBAADF4BNWFqN/lY+n2HRlHEXsUzsSqUebrj2RzwUYKzA0fgFMSKtWBIh4QOhkYc0fN90FE3s0PVD8l1pa3eQgKm4Q/Qt9AAAAAAAAIHS9QQasHv3F3Mu4wOC3g+XyVZWVNag38n+RDUyb5r6TxeATVhajf5WPp9h0ZRxF7FM7EqlHm649kc8FGCswNH7oAwAAAAAAAADyBSoBAAAAAAFhAPcnecANgbSE8RcdAb9+s/SVzZGx0KG5xQkqzy1Fdj5N5gMDwjNMPIR4rrrK8dEB2M6AFpShBr5bbpFNsuRTPgFrtgsyizxsLSk5CcFnPJ/SutuzHRewjc62V245E/XCMw==";
    // let bcs = base64ct::Base64::decode_vec(&bcs).unwrap();
    // let tx: SignedTransaction = bcs::from_bytes(&bcs).unwrap();
    // println!("{:?}", tx);
}
