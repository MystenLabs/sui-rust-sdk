#[test]
fn compile_tests() {
    let t = trybuild::TestCases::new();

    // Tests that should pass
    t.pass("tests/compile/basic_extraction.rs");

    // Tests that should fail with expected errors
    t.compile_fail("tests/compile/missing_field_path_attr.rs");
    t.compile_fail("tests/compile/missing_field_attr.rs");
    t.compile_fail("tests/compile/enum_not_supported.rs");
    t.compile_fail("tests/compile/empty_path.rs");
    t.compile_fail("tests/compile/tuple_struct_not_supported.rs");
}
