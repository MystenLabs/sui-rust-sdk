//! Example demonstrating dynamic field queries.
//!
//! Run with: cargo run -p sui-graphql --example dynamic_fields

use futures::StreamExt;
use std::pin::pin;
use sui_graphql::Bcs;
use sui_graphql::Client;
use sui_sdk_types::Address;
use sui_sdk_types::TypeTag;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("https://graphql.testnet.sui.io/graphql")?;

    // Example: Query an object that has dynamic fields
    // You can replace this with any object ID that has dynamic fields
    let parent: Address =
        "0x109e64e019a2d236822e904e5663f6882921475e7f3b7b14f7fa4cd8ac2846a3".parse()?;

    println!("=== Listing all dynamic fields ===");
    let mut stream = pin!(client.get_dynamic_fields(parent));
    let mut count = 0;
    while let Some(result) = stream.next().await {
        match result {
            Ok(entry) => {
                count += 1;
                println!("Field #{count}:");
                println!("  name_type: {}", entry.name_type);
                println!("  name: {}", entry.name);
                println!("  value_type: {}", entry.value_type);
                println!("  value: {}", entry.value);
                println!("  field_type: {:?}", entry.field_type);
                println!();
            }
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
        }
    }
    println!("Total fields: {count}");

    println!("\n=== Point lookup for a specific dynamic field ===");
    // Look up a dynamic field with a vector<u8> name (matching what we found above)
    // The name "YWI=" is base64 for "ab", so we look up b"ab"
    let name_type: TypeTag = "vector<u8>".parse()?;
    let name_value: Vec<u8> = b"ab".to_vec();

    println!("Looking up: name_type={name_type}, name={name_value:?}");

    match client
        .get_dynamic_object_field(parent, name_type.clone(), Bcs(name_value.clone()))
        .await
    {
        Ok(Some(entry)) => {
            println!("Found dynamic object field:");
            println!("  name_type: {}", entry.name_type);
            println!("  name: {}", entry.name);
            println!("  value_type: {}", entry.value_type);
            println!("  value: {}", entry.value);
            println!("  field_type: {:?}", entry.field_type);
        }
        Ok(None) => {
            println!(
                "Dynamic object field not found for name_type={name_type}, name={name_value:?}"
            );
        }
        Err(e) => {
            eprintln!("Error: {e}");
        }
    }

    Ok(())
}
