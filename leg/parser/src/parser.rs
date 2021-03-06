pub(crate) mod marker;
mod parse_error;

use crate::event::Event;
use crate::grammar;
use crate::source::Source;
use lexer::{Token, TokenKind};
use marker::Marker;
pub(crate) use parse_error::ParseError;
use std::mem;
use syntax::syntax_kind::SyntaxKind;

const RECOVERY_SET: [TokenKind; 1] = [TokenKind::LetKeyword];

pub struct Parser<'t, 'input> {
    source: Source<'t, 'input>,
    events: Vec<Event>,
    expected_kinds: Vec<TokenKind>,
}

impl<'t, 'input> Parser<'t, 'input> {
    pub(crate) fn new(tokens: &'t [Token<'input>]) -> Self {
        Self {
            source: Source::new(tokens),
            events: Vec::new(),
            expected_kinds: Vec::new(),
        }
    }

    pub(crate) fn parse(mut self) -> Vec<Event> {
        grammar::root(&mut self);
        self.events
    }

    pub(crate) fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::tombstone());
        Marker::new(pos)
    }

    pub(crate) fn bump(&mut self) {
        self.expected_kinds.clear();
        self.source.next_token().unwrap();
        self.events.push(Event::AddToken);
    }

    pub(crate) fn eat(&mut self, kind: TokenKind) -> bool {
        if self.peek() == Some(kind) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn peek(&mut self) -> Option<TokenKind> {
        self.source.peek_kind()
    }

    pub(crate) fn at(&mut self, kind: TokenKind) -> bool {
        self.expected_kinds.push(kind);
        self.peek() == Some(kind)
    }

    pub(crate) fn expect(&mut self, kind: TokenKind) {
        if self.at(kind) {
            self.bump();
        } else {
            self.error();
        }
    }

    pub(crate) fn error(&mut self) {
        let current_token = self.source.peek_token();

        let (found, range) = if let Some(Token { kind, range, .. }) = current_token {
            (Some(*kind), *range)
        } else {
            (None, self.source.last_token_range().unwrap())
        };

        self.events.push(Event::Error(ParseError {
            expected: mem::take(&mut self.expected_kinds),
            found,
            range,
        }));

        if !self.at_set(&RECOVERY_SET) && !self.at_end() {
            let marker = self.start();
            self.bump();
            marker.complete(self, SyntaxKind::Error);
        }
    }

    pub(crate) fn at_set(&mut self, set: &[TokenKind]) -> bool {
        self.peek().map_or(false, |k| set.contains(&k))
    }

    pub(crate) fn at_end(&mut self) -> bool {
        self.peek().is_none()
    }
}

#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn parse_nothing() {
        check("", expect![[r#"Root@0..0"#]])
    }

    #[test]
    fn parse_whitespace() {
        check(
            "   ",
            expect![[r#"
        Root@0..3
          Whitespace@0..3 "   ""#]],
        )
    }

    #[test]
    fn parse_comment() {
        check(
            "// hello",
            expect![[r#"
        Root@0..8
          Comment@0..8 "// hello""#]],
        )
    }
}
