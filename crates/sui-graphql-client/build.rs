/// Register Sui RPC schema for creating structs for queries
fn main() {
    sui_graphql_client_build::register_schema("rpc");
}
