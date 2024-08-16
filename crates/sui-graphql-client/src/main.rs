use base64ct::Encoding;
use sui_graphql_client::SuiClient;
use sui_types::types::SignedTransaction;
#[tokio::main]
async fn main() {
    let mut client = SuiClient::default();
    // client.set_mainnet();
    client.set_devnet();
    client.set_version(Some("beta"));
    let total_checkpoints = client.epoch_total_checkpoints(None).await;
    let epoch_summary = client.epoch_total_transaction_blocks(None).await;
    println!("{:?}", total_checkpoints);
    println!("{:?}", epoch_summary);

    // let tx = client
    // .transaction("6u6UHNpBX1qiesAL3fhhGzCfSmCzfV3VyrgJXkEcNAAM")
    // .await;

    // println!("{:?}", tx);

    let bcs = "AQAAAAAAAQEBkAwKIXjgdBxrTtnPV0Kg/imCwo3W6vK+402SUxjvG3wEAAAAAAAAAAEBABzcym2+gR0xpC9SRrR4GlJn5UnXy9/92SHFq+YQ/jf+B2NvdW50ZXIJaW5jcmVtZW50AAEBAADF4BNWFqN/lY+n2HRlHEXsUzsSqUebrj2RzwUYKzA0fgFMSKtWBIh4QOhkYc0fN90FE3s0PVD8l1pa3eQgKm4Q/Qt9AAAAAAAAIHS9QQasHv3F3Mu4wOC3g+XyVZWVNag38n+RDUyb5r6TxeATVhajf5WPp9h0ZRxF7FM7EqlHm649kc8FGCswNH7oAwAAAAAAAADyBSoBAAAAAAFhAPcnecANgbSE8RcdAb9+s/SVzZGx0KG5xQkqzy1Fdj5N5gMDwjNMPIR4rrrK8dEB2M6AFpShBr5bbpFNsuRTPgFrtgsyizxsLSk5CcFnPJ/SutuzHRewjc62V245E/XCMw==";
    let bcs = base64ct::Base64::decode_vec(&bcs).unwrap();
    let tx: SignedTransaction = bcs::from_bytes(&bcs).unwrap();
    println!("{:?}", tx);
}
