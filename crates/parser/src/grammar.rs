mod expr;
mod stmt;

use crate::parser::marker::CompletedMarker;
use crate::parser::Parser;
use lexer::TokenKind;
use syntax::SyntaxKind;

pub(super) fn root(parser: &mut Parser) -> CompletedMarker {
    let marker = parser.start();
    while !parser.at_end() {
        stmt::parse_statement(parser);
    }
    marker.complete(parser, SyntaxKind::Root)
}
