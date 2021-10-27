use super::expr::parse_expression;
use super::CompletedMarker;
use super::Parser;
use lexer::TokenKind;
use syntax::SyntaxKind;

pub(super) fn parse_statement(parser: &mut Parser) -> Option<CompletedMarker> {
    if parser.at(TokenKind::LetKeyword) {
        Some(parse_variable_def(parser))
    } else {
        parse_expression(parser)
    }
}

fn parse_variable_def(parser: &mut Parser) -> CompletedMarker {
    assert!(parser.at(TokenKind::LetKeyword));
    let marker = parser.start();

    // Eat let keyword
    parser.bump();

    // Eat mut keyword if present
    if parser.at(TokenKind::MutKeyword) {
        parser.bump();
    }

    parser.expect(TokenKind::Ident);
    parser.expect(TokenKind::Equals);

    parse_expression(parser);

    marker.complete(parser, SyntaxKind::VariableDef)
}

#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn parse_variable_definition() {
        check(
            "let foo = bar",
            expect![[r#"
Root@0..13
  VariableDef@0..13
    LetKeyword@0..3 "let"
    Whitespace@3..4 " "
    Ident@4..7 "foo"
    Whitespace@7..8 " "
    Equals@8..9 "="
    Whitespace@9..10 " "
    VariableRef@10..13
      Ident@10..13 "bar""#]],
        );
    }

    #[test]
    fn parse_mutable_variable_definition() {
        check(
            "let mut foo = bar",
            expect![[r#"
Root@0..17
  VariableDef@0..17
    LetKeyword@0..3 "let"
    Whitespace@3..4 " "
    MutKeyword@4..7 "mut"
    Whitespace@7..8 " "
    Ident@8..11 "foo"
    Whitespace@11..12 " "
    Equals@12..13 "="
    Whitespace@13..14 " "
    VariableRef@14..17
      Ident@14..17 "bar""#]],
        )
    }

    #[test]
    fn recover_on_let_token() {
        check(
            "let a =\nlet b = a",
            expect![[r#"
Root@0..17
  VariableDef@0..8
    LetKeyword@0..3 "let"
    Whitespace@3..4 " "
    Ident@4..5 "a"
    Whitespace@5..6 " "
    Equals@6..7 "="
    Whitespace@7..8 "\n"
  VariableDef@8..17
    LetKeyword@8..11 "let"
    Whitespace@11..12 " "
    Ident@12..13 "b"
    Whitespace@13..14 " "
    Equals@14..15 "="
    Whitespace@15..16 " "
    VariableRef@16..17
      Ident@16..17 "a"
error at 8..11: expected number, identifier, `-` or `(`, but found let"#]],
        );
    }

    #[test]
    fn parse_multiple_statements() {
        check(
            "let a = 1\na",
            expect![[r#"
Root@0..11
  VariableDef@0..10
    LetKeyword@0..3 "let"
    Whitespace@3..4 " "
    Ident@4..5 "a"
    Whitespace@5..6 " "
    Equals@6..7 "="
    Whitespace@7..8 " "
    Literal@8..10
      Integer@8..9 "1"
      Whitespace@9..10 "\n"
  VariableRef@10..11
    Ident@10..11 "a""#]],
        );
    }
}
