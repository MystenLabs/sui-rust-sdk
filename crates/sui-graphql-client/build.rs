/// Register Sui RPC schema for creating structs for queries
fn main() {
    #[cfg(feature = "staging")]
    cynic_codegen::register_schema("rpc")
        .from_sdl_file("schema/graphql_rpc_staging.graphql")
        .expect("Failed to find GraphQL Schema");
    #[cfg(not(feature = "staging"))]
    cynic_codegen::register_schema("rpc")
        .from_sdl_file("schema/graphql_rpc.graphql")
        .expect("Failed to find GraphQL Schema");
}
