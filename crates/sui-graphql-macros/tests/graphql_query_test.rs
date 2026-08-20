use sui_graphql_macros::graphql_query;

const FILE_QUERY: &str = graphql_query!(
    @"queries/chain_identifier_query.graphql",
    @"queries/chain_identifier_fragment.graphql",
);

const MIXED_QUERY: &str = graphql_query!(
    "query { ...ChainIdentifier }",
    @"queries/chain_identifier_fragment.graphql",
);

#[test]
fn file_sources_are_loaded_and_formatted() {
    assert_eq!(FILE_QUERY, MIXED_QUERY);
    assert!(FILE_QUERY.contains("chainIdentifier"));
}
