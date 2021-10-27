use super::*;

enum UnaryOp {
    Negate,
}

impl UnaryOp {
    pub fn binding_power(&self) -> ((), u8) {
        match self {
            UnaryOp::Negate => ((), 5),
        }
    }
}

enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinaryOp {
    pub fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Add | Self::Sub => (1, 2),
            Self::Mul | Self::Div => (3, 4),
        }
    }
}

/// Parses an expression
pub(super) fn parse_expression(parser: &mut Parser) -> Option<CompletedMarker> {
    parse_expression_bp(parser, 0)
}

/// Parses an expression with the given binding power
fn parse_expression_bp(parser: &mut Parser, minimum_binding_power: u8) -> Option<CompletedMarker> {
    let mut lhs = parse_left_hand_side(parser)?;

    loop {
        let op = if parser.at(TokenKind::Plus) {
            BinaryOp::Add
        } else if parser.at(TokenKind::Minus) {
            BinaryOp::Sub
        } else if parser.at(TokenKind::Star) {
            BinaryOp::Mul
        } else if parser.at(TokenKind::Slash) {
            BinaryOp::Div
        } else {
            break;
        };

        let (left_binding_power, right_binding_power) = op.binding_power();

        if left_binding_power < minimum_binding_power {
            break;
        }

        // Eat the operator's token.
        parser.bump();

        let marker = lhs.precede(parser);
        let parsed_rhs = parse_expression_bp(parser, right_binding_power).is_some();
        lhs = marker.complete(parser, SyntaxKind::InfixExpr);

        if !parsed_rhs {
            break;
        }
    }

    Some(lhs)
}

/// Parses the left hand side of an expression
fn parse_left_hand_side(parser: &mut Parser) -> Option<CompletedMarker> {
    let cm = if parser.at(TokenKind::Integer) {
        parse_literal(parser)
    } else if parser.at(TokenKind::String) {
        parse_literal(parser)
    } else if parser.at(TokenKind::True) {
        parse_literal(parser)
    } else if parser.at(TokenKind::False) {
        parse_literal(parser)
    } else if parser.at(TokenKind::Ident) {
        parse_variable_ref(parser)
    } else if parser.at(TokenKind::Minus) {
        parse_prefix_expression(parser)
    } else if parser.at(TokenKind::LParen) {
        parse_paren_expression(parser)
    } else {
        parser.error();
        return None;
    };

    Some(cm)
}

const LITERAL_FIRST: &[TokenKind] = &[
    TokenKind::Integer,
    TokenKind::String,
    TokenKind::True,
    TokenKind::False,
];

/// Parses a literal expression
fn parse_literal(parser: &mut Parser) -> CompletedMarker {
    assert!(parser.at_set(LITERAL_FIRST));

    let marker = parser.start();
    parser.bump();
    marker.complete(parser, SyntaxKind::Literal)
}

fn parse_variable_ref(parser: &mut Parser) -> CompletedMarker {
    assert!(parser.at(TokenKind::Ident));

    let marker = parser.start();
    parser.bump();
    marker.complete(parser, SyntaxKind::VariableRef)
}

fn parse_prefix_expression(parser: &mut Parser) -> CompletedMarker {
    assert!(parser.at(TokenKind::Minus));

    let marker = parser.start();

    let op = UnaryOp::Negate;
    let ((), right_binding_power) = op.binding_power();

    parser.bump();

    parse_expression_bp(parser, right_binding_power);

    marker.complete(parser, SyntaxKind::PrefixExpr)
}

fn parse_paren_expression(parser: &mut Parser) -> CompletedMarker {
    assert!(parser.at(TokenKind::LParen));

    let marker = parser.start();

    parser.bump();
    parse_expression_bp(parser, 0);
    parser.expect(TokenKind::RParen);

    marker.complete(parser, SyntaxKind::ParenExpr)
}

#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn parse_true() {
        check(
            "true",
            expect![[r#"
Root@0..4
  Literal@0..4
    True@0..4 "true""#]],
        )
    }

    #[test]
    fn parse_false() {
        check(
            "false",
            expect![[r#"
Root@0..5
  Literal@0..5
    False@0..5 "false""#]],
        )
    }

    #[test]
    fn parse_string() {
        check(
            "\"abc\"",
            expect![[r#"
Root@0..5
  Literal@0..5
    String@0..5 "\"abc\"""#]],
        )
    }

    #[test]
    fn parse_number() {
        check(
            "123",
            expect![[r#"
Root@0..3
  Literal@0..3
    Integer@0..3 "123""#]],
        );
    }

    #[test]
    fn parse_number_preceded_by_whitespace() {
        check(
            "   9876",
            expect![[r#"
Root@0..7
  Whitespace@0..3 "   "
  Literal@3..7
    Integer@3..7 "9876""#]],
        );
    }

    #[test]
    fn parse_number_followed_by_whitespace() {
        check(
            "999   ",
            expect![[r#"
Root@0..6
  Literal@0..6
    Integer@0..3 "999"
    Whitespace@3..6 "   ""#]],
        );
    }

    #[test]
    fn parse_number_surrounded_by_whitespace() {
        check(
            " 123     ",
            expect![[r#"
Root@0..9
  Whitespace@0..1 " "
  Literal@1..9
    Integer@1..4 "123"
    Whitespace@4..9 "     ""#]],
        );
    }

    #[test]
    fn parse_variable_ref() {
        check(
            "counter",
            expect![[r#"
Root@0..7
  VariableRef@0..7
    Ident@0..7 "counter""#]],
        );
    }

    #[test]
    fn parse_simple_infix_expression() {
        check(
            "1+2",
            expect![[r#"
Root@0..3
  InfixExpr@0..3
    Literal@0..1
      Integer@0..1 "1"
    Plus@1..2 "+"
    Literal@2..3
      Integer@2..3 "2""#]],
        )
    }

    #[test]
    fn parse_left_associative_infix_expression() {
        check(
            "1+2+3+4",
            expect![[r#"
Root@0..7
  InfixExpr@0..7
    InfixExpr@0..5
      InfixExpr@0..3
        Literal@0..1
          Integer@0..1 "1"
        Plus@1..2 "+"
        Literal@2..3
          Integer@2..3 "2"
      Plus@3..4 "+"
      Literal@4..5
        Integer@4..5 "3"
    Plus@5..6 "+"
    Literal@6..7
      Integer@6..7 "4""#]],
        );
    }

    #[test]
    fn parse_infix_expression_with_mixed_binding_power() {
        check(
            "1+2*3-4",
            expect![[r#"
Root@0..7
  InfixExpr@0..7
    InfixExpr@0..5
      Literal@0..1
        Integer@0..1 "1"
      Plus@1..2 "+"
      InfixExpr@2..5
        Literal@2..3
          Integer@2..3 "2"
        Star@3..4 "*"
        Literal@4..5
          Integer@4..5 "3"
    Minus@5..6 "-"
    Literal@6..7
      Integer@6..7 "4""#]],
        );
    }

    #[test]
    fn parse_infix_expression_with_whitespace() {
        check(
            " 1 +   2* 3 ",
            expect![[r#"
Root@0..12
  Whitespace@0..1 " "
  InfixExpr@1..12
    Literal@1..3
      Integer@1..2 "1"
      Whitespace@2..3 " "
    Plus@3..4 "+"
    Whitespace@4..7 "   "
    InfixExpr@7..12
      Literal@7..8
        Integer@7..8 "2"
      Star@8..9 "*"
      Whitespace@9..10 " "
      Literal@10..12
        Integer@10..11 "3"
        Whitespace@11..12 " ""#]],
        );
    }

    #[test]
    fn parse_infix_expression_interspersed_with_comments() {
        check(
            "
1
  + 1 // Add one
  + 10 // Add ten",
            expect![[r#"
Root@0..37
  Whitespace@0..1 "\n"
  InfixExpr@1..37
    InfixExpr@1..22
      Literal@1..5
        Integer@1..2 "1"
        Whitespace@2..5 "\n  "
      Plus@5..6 "+"
      Whitespace@6..7 " "
      Literal@7..22
        Integer@7..8 "1"
        Whitespace@8..9 " "
        Comment@9..19 "// Add one"
        Whitespace@19..22 "\n  "
    Plus@22..23 "+"
    Whitespace@23..24 " "
    Literal@24..37
      Integer@24..26 "10"
      Whitespace@26..27 " "
      Comment@27..37 "// Add ten""#]],
        );
    }

    #[test]
    fn parse_negation() {
        check(
            "-10",
            expect![[r#"
Root@0..3
  PrefixExpr@0..3
    Minus@0..1 "-"
    Literal@1..3
      Integer@1..3 "10""#]],
        )
    }

    #[test]
    fn negation_has_higher_binding_power_than_infix_operators() {
        check(
            "-10+20",
            expect![[r#"
Root@0..6
  InfixExpr@0..6
    PrefixExpr@0..3
      Minus@0..1 "-"
      Literal@1..3
        Integer@1..3 "10"
    Plus@3..4 "+"
    Literal@4..6
      Integer@4..6 "20""#]],
        )
    }

    #[test]
    fn parse_nested_parentheses() {
        check(
            "((((10))))",
            expect![[r#"
Root@0..10
  ParenExpr@0..10
    LParen@0..1 "("
    ParenExpr@1..9
      LParen@1..2 "("
      ParenExpr@2..8
        LParen@2..3 "("
        ParenExpr@3..7
          LParen@3..4 "("
          Literal@4..6
            Integer@4..6 "10"
          RParen@6..7 ")"
        RParen@7..8 ")"
      RParen@8..9 ")"
    RParen@9..10 ")""#]],
        )
    }

    #[test]
    fn parentheses_affect_precedence() {
        check(
            "5*(2+1)",
            expect![[r#"
Root@0..7
  InfixExpr@0..7
    Literal@0..1
      Integer@0..1 "5"
    Star@1..2 "*"
    ParenExpr@2..7
      LParen@2..3 "("
      InfixExpr@3..6
        Literal@3..4
          Integer@3..4 "2"
        Plus@4..5 "+"
        Literal@5..6
          Integer@5..6 "1"
      RParen@6..7 ")""#]],
        )
    }

    #[test]
    fn parse_unclosed_parentheses() {
        check(
            "(foo",
            expect![[r#"
Root@0..4
  ParenExpr@0..4
    LParen@0..1 "("
    VariableRef@1..4
      Ident@1..4 "foo"
error at 1..4: expected `+`, `-`, `*`, `/` or `)`"#]],
        );
    }

    #[test]
    fn do_not_parse_operator_if_gettting_rhs_failed() {
        check(
            "(1+",
            expect![[r#"
Root@0..3
  ParenExpr@0..3
    LParen@0..1 "("
    InfixExpr@1..3
      Literal@1..2
        Integer@1..2 "1"
      Plus@2..3 "+"
error at 2..3: expected number, string, true, false, identifier, `-` or `(`
error at 2..3: expected `)`"#]],
        );
    }
}
