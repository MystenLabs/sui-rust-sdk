use sui_graphql::scalars::Digest;

#[tokio::main]
async fn main() {
    // Test parsing the raw value
    let raw = "35834a8a";
    match raw.parse::<Digest>() {
        Ok(d) => {
            println!("Parsed '{}' as Digest: {:?}", raw, d);
            let bytes = d.as_bytes();
            print!("Digest bytes (hex): ");
            for b in bytes {
                print!("{:02x}", b);
            }
            println!();
        }
        Err(e) => println!("Failed to parse '{}': {:?}", raw, e),
    }
    
    // Also check the real genesis checkpoint digest
    let genesis = "4btiuiMPvEENsttpZC7CZ53DruC3MAgfznDbASZ7DR6S";
    match genesis.parse::<Digest>() {
        Ok(d) => {
            println!("\nParsed genesis '{}' as Digest", genesis);
            let bytes = d.as_bytes();
            print!("Genesis bytes (hex): ");
            for b in bytes {
                print!("{:02x}", b);
            }
            println!();
        }
        Err(e) => println!("Failed to parse genesis: {:?}", e),
    }
}
