mod token_kind;

use logos::Logos;
use std::ops::Range as StdRange;
use text_size::{TextRange, TextSize};
pub use token_kind::TokenKind;

pub struct Lexer<'a> {
    inner: logos::Lexer<'a, TokenKind>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        Lexer {
            inner: TokenKind::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice();

        let range = {
            let StdRange { start, end } = self.inner.span();
            let start = TextSize::try_from(start).unwrap();
            let end = TextSize::try_from(end).unwrap();

            TextRange::new(start, end)
        };

        Some(Self::Item { kind, text, range })
    }
}

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub text: &'a str,
    pub range: TextRange,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, kind: TokenKind) {
        let mut lexer = Lexer::new(input);

        let token = lexer.next().unwrap();
        assert_eq!(token.kind, kind);
        assert_eq!(token.text, input);
    }

    #[test]
    fn lex_spaces_and_newlines() {
        check("  \n ", TokenKind::Whitespace);
    }

    #[test]
    fn lex_spaces() {
        check("   ", TokenKind::Whitespace);
    }

    #[test]
    fn lex_comma() {
        check(",", TokenKind::Comma);
    }

    #[test]
    fn lex_semicolon() {
        check(";", TokenKind::Semicolon);
    }

    #[test]
    fn lex_colon() {
        check(":", TokenKind::Colon);
    }

    #[test]
    fn lex_func() {
        check("func", TokenKind::FuncKeyword);
    }

    #[test]
    fn lex_comp() {
        check("comp", TokenKind::CompKeyword);
    }

    #[test]
    fn lex_percent() {
        check("%", TokenKind::Percent);
    }

    #[test]
    fn lex_single_character_identifier() {
        check("a", TokenKind::Ident);
    }

    #[test]
    fn lex_alphabetic_identifier() {
        check("abcd", TokenKind::Ident);
    }

    #[test]
    fn lex_alphanumeric_identifier() {
        check("abcd1234", TokenKind::Ident);
    }

    #[test]
    fn lex_underscore_identifier() {
        check("__abc123", TokenKind::Ident);
    }

    #[test]
    fn lex_string_literal() {
        check("\"hello world\"", TokenKind::String);
    }

    #[test]
    fn lex_number() {
        check("12345", TokenKind::Integer);
    }

    #[test]
    fn lex_plus() {
        check("+", TokenKind::Plus);
    }

    #[test]
    fn lex_minus() {
        check("-", TokenKind::Minus);
    }

    #[test]
    fn lex_star() {
        check("*", TokenKind::Star);
    }

    #[test]
    fn lex_slash() {
        check("/", TokenKind::Slash);
    }

    #[test]
    fn lex_equals() {
        check("=", TokenKind::Equals);
    }

    #[test]
    fn lex_bang() {
        check("!", TokenKind::Bang);
    }

    #[test]
    fn lex_bang_equals() {
        check("!=", TokenKind::BangEquals);
    }

    #[test]
    fn lex_less_than() {
        check("<", TokenKind::LessThan);
    }

    #[test]
    fn lex_less_than_equals() {
        check("<=", TokenKind::LessThanEqual);
    }

    #[test]
    fn lex_greater_than() {
        check(">", TokenKind::GreaterThan);
    }

    #[test]
    fn lex_greater_than_equals() {
        check(">=", TokenKind::GreaterThanEqual);
    }

    #[test]
    fn lex_ampersand_ampersand() {
        check("&&", TokenKind::AmpersandAmpersand);
    }

    #[test]
    fn lex_bar_bar() {
        check("||", TokenKind::BarBar);
    }

    #[test]
    fn lex_left_paren() {
        check("(", TokenKind::LParen);
    }

    #[test]
    fn lex_right_paren() {
        check(")", TokenKind::RParen);
    }

    #[test]
    fn lex_left_brace() {
        check("{", TokenKind::LBrace);
    }

    #[test]
    fn lex_right_brace() {
        check("}", TokenKind::RBrace);
    }

    #[test]
    fn lex_left_bracket() {
        check("[", TokenKind::LBracket);
    }

    #[test]
    fn lex_right_bracket() {
        check("]", TokenKind::RBracket);
    }

    #[test]
    fn lex_effect_keyword() {
        check("effect", TokenKind::EffectKeyword);
    }

    #[test]
    fn lex_on_mount_keyword() {
        check("onMount", TokenKind::OnMountKeyword);
    }

    #[test]
    fn lex_on_update_keyword() {
        check("onUpdate", TokenKind::OnUpdateKeyword);
    }

    #[test]
    fn lex_on_destroy_keyword() {
        check("onDestroy", TokenKind::OnDestroyKeyword);
    }

    #[test]
    fn lex_comment() {
        check("// foo", TokenKind::Comment);
    }
}
