use css_lexer::{Lexer, TokenKind};
use css_parser::{Parser, TokenCursor};
use cursor::StringCursor;

#[test]
fn test_selectors() {
    let cursor = StringCursor::new(".blue#red {}");
    let mut lexer = Lexer::new(cursor);
    let mut tokens = Vec::new();
    while let Ok(token) = lexer.next() {
        if token.kind == TokenKind::EOF {
            tokens.push(token);
            break;
        } else {
            tokens.push(token);
        }
    }
    let token_cursor = TokenCursor::new(tokens);
    Parser::new(token_cursor).parse().unwrap();
}
