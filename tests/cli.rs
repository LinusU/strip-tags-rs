#[test]
fn cli_tests() {
    // Test examples in the top-level readme
    trycmd::TestCases::new().case("readme.md");
}
