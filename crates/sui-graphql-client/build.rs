/// Register Sui RPC schema for creating structs for queries
fn main() {
    cynic_codegen::register_schema("rpc")
        .from_sdl_file("schema/graphql_rpc.graphql")
        .expect("Failed to find GraphQL Schema");
}
