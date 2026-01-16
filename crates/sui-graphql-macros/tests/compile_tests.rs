#[test]
fn compile_tests() {
    let t = trybuild::TestCases::new();

    // Tests that should pass
    t.pass("tests/compile/basic_extraction.rs");
    t.pass("tests/compile/skip_validation.rs");

    // Tests that should fail with expected errors
    t.compile_fail("tests/compile/missing_field_path_attr.rs");
    t.compile_fail("tests/compile/missing_field_attr.rs");
    t.compile_fail("tests/compile/enum_not_supported.rs");
    t.compile_fail("tests/compile/empty_path.rs");

    // Schema validation tests
    t.compile_fail("tests/compile/schema_validation_field_not_found.rs");
    t.compile_fail("tests/compile/schema_validation_array_on_non_list.rs");
    t.compile_fail("tests/compile/schema_validation_field_after_list_not_found.rs");
}
