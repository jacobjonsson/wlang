use ast::Component;
use ast::Script;
use ast::Style;
use ast::View;
use ast::ViewNode;
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
    );

    assert_eq!(
        Parser::new("view {<div />}").parse().unwrap(),
        Component {
            script: { Script { statements: vec![] } },
            style: { Style { rules: vec![] } },
            view: {
                View {
                    children: vec![ViewNode::Element {
                        attributes: vec![],
                        name: "div".into(),
                        children: vec![],
                    }],
                }
            }
        }
    );

    assert_eq!(
        Parser::new("view {<div></div>}").parse().unwrap(),
        Component {
            script: { Script { statements: vec![] } },
            style: { Style { rules: vec![] } },
            view: {
                View {
                    children: vec![ViewNode::Element {
                        attributes: vec![],
                        name: "div".into(),
                        children: vec![],
                    }],
                }
            }
        }
    )
}
