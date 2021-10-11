use super::Parser;

#[test]
fn test_declaration() {
    insta::assert_json_snapshot!(Parser::new("fn myFunction() {}").parse())
}
