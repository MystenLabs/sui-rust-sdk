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

#[test]
fn file_comments_at_eof_are_terminated() {
    // Keep the fixture without a final newline to exercise the source boundary.
    assert!(
        !include_str!("queries/chain_identifier_fragment_with_comment.graphql").ends_with('\n')
    );

    let expected = graphql_query!(
        "fragment ChainIdentifier on Query { chainIdentifier }",
        "query { ...ChainIdentifier }",
    );

    let followed_by_inline = graphql_query!(
        @"queries/chain_identifier_fragment_with_comment.graphql",
        "query { ...ChainIdentifier }",
    );

    let followed_by_file = graphql_query!(
        @"queries/chain_identifier_fragment_with_comment.graphql",
        @"queries/chain_identifier_query.graphql",
    );

    assert_eq!(followed_by_inline, expected);
    assert_eq!(followed_by_file, expected);
}
