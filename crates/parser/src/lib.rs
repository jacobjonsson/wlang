mod event;
mod grammar;
mod parser;
mod sink;
mod source;

use crate::parser::ParseError;
use crate::parser::Parser;
use lexer::Lexer;
use rowan::GreenNode;
use sink::Sink;
use syntax::SyntaxNode;

pub fn parse(input: &str) -> Parse {
    let tokens: Vec<_> = Lexer::new(input).collect();
    let parser = Parser::new(&tokens);
    let events = parser.parse();
    let sink = Sink::new(&tokens, events);
    sink.finish()
}

pub struct Parse {
    green_node: GreenNode,
    errors: Vec<ParseError>,
}

impl Parse {
    pub fn debug_tree(&self) -> String {
        let mut s = String::new();

        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let tree = format!("{:#?}", syntax_node);
        s.push_str(&tree[0..tree.len() - 1]);

        for err in &self.errors {
            s.push_str(&format!("\n{}", err));
        }

        s
    }

    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }
}

#[cfg(test)]
pub(crate) fn check(input: &str, expected_tree: expect_test::Expect) {
    let parse = parse(input);
    expected_tree.assert_eq(&parse.debug_tree());
}
