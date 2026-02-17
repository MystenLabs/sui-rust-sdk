#[test]
fn compile_tests() {
    // Set the schema directory for trybuild tests (trybuild copies files to a temp directory,
    // so we need to tell the proc macro where to find schema files relative to)
    // SAFETY: This test runs single-threaded and no other code reads this env var concurrently.
    unsafe {
        std::env::set_var("SUI_GRAPHQL_SCHEMA_DIR", env!("CARGO_MANIFEST_DIR"));
    }

    let t = trybuild::TestCases::new();

    // Tests that should pass
    t.pass("tests/compile/basic_extraction.rs");
    t.pass("tests/compile/skip_validation.rs");
    t.pass("tests/compile/schema_validation_with_alias.rs");
    t.pass("tests/compile/root_type_mutation.rs");
    t.pass("tests/compile/root_type_dynamic_field.rs");

    // Tests that should fail with expected errors
    t.compile_fail("tests/compile/missing_field_path_attr.rs");
    t.compile_fail("tests/compile/missing_field_attr.rs");
    t.compile_fail("tests/compile/enum_not_supported.rs");
    t.compile_fail("tests/compile/empty_path.rs");
    t.compile_fail("tests/compile/tuple_struct_not_supported.rs");

    // Schema validation tests
    t.compile_fail("tests/compile/schema_validation_field_not_found.rs");
    t.compile_fail("tests/compile/schema_validation_field_suggestion.rs");
    t.compile_fail("tests/compile/schema_validation_array_field_suggestion.rs");
    t.compile_fail("tests/compile/schema_validation_field_after_list_suggestion.rs");
    t.compile_fail("tests/compile/schema_validation_array_on_non_list.rs");
    t.compile_fail("tests/compile/schema_validation_missing_array_marker.rs");
    t.compile_fail("tests/compile/schema_validation_field_after_list_not_found.rs");
    t.compile_fail("tests/compile/schema_validation_alias_invalid_field.rs");

    // Type validation tests
    t.compile_fail("tests/compile/type_validation_vec_count_mismatch.rs");
    t.compile_fail("tests/compile/type_validation_trailing_array_mismatch.rs");
    t.pass("tests/compile/skip_validation_type_mismatch.rs");

    // Root type validation tests
    t.compile_fail("tests/compile/root_type_not_found.rs");
    t.compile_fail("tests/compile/root_type_typo_suggestion.rs");
}
