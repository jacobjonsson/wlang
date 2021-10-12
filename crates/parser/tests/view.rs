use ast::Component;
use ast::Script;
use ast::Style;
use ast::View;
use parser::Parser;

#[test]
fn test_basic_view() {
    assert_eq!(
        Parser::new("view {}").parse().unwrap(),
        Component {
            script: { Script { statements: vec![] } },
            style: { Style { rules: vec![] } },
            view: { View { children: vec![] } }
        }
    )
}
