// Build-only programmable transaction example (no execution).
// Run: cargo run --example transaction_build
use sui_sdk_types::{Address, TypeTag};
use sui_transaction_builder::{Function, Serialized, TransactionBuilder};

fn pkg() -> Address {
    "0x1".parse().unwrap()
}

fn main() {
    let mut b = TransactionBuilder::new();
    let arg = b.input(Serialized(&vec![1u64]));
    let f = Function::new(
        pkg(),
        "option".parse().unwrap(),
        "is_none".parse().unwrap(),
        vec![TypeTag::U64],
    );
    b.move_call(f, vec![arg]);
    b.set_sender(
        "0x1111111111111111111111111111111111111111111111111111111111111111"
            .parse()
            .unwrap(),
    );
    match b.finish() {
        Ok(tx) => println!("Digest: {:?}", tx.digest()),
        Err(e) => println!("Finish error (expected): {e}"),
    }
}
