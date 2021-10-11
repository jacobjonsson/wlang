use crate::{Lexer, LexerResult};
use token::TokenKind;

/// True if `c` is considered an identifier start.
pub fn is_identifier_start(c: char) -> bool {
    ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || c == '_' || c == '$'
}

/// True if `c` is considered an identifier start continuation.
pub fn is_identifier_continue(c: char) -> bool {
    ('a'..='z').contains(&c)
        || ('A'..='Z').contains(&c)
        || ('0'..='9').contains(&c)
        || c == '\u{200C}'
        || c == '\u{200D}'
        || c == '_'
        || c == '$'
}

impl<'a> Lexer<'a> {
    pub(crate) fn scan_identifier(&mut self) -> LexerResult<TokenKind> {
        let start = self.cursor.current_position();
        loop {
            let character = match self.cursor.current() {
                Some(c) => c,
                None => break,
            };

            if !is_identifier_continue(character) {
                break;
            }

            self.cursor.bump();
        }

        let end = self.cursor.current_position();
        let identifier = self.cursor.slice(start, end);
        Ok(TokenKind::from_string(identifier.into()))
    }
}

#[cfg(test)]
mod tests {
    use crate::Lexer;
    use token::TokenKind;

    #[test]
    fn test_valid_identifiers() {
        let tests = [("a;", "a"), ("_a", "_a"), ("$a", "$a"), ("a_b", "a_b")];

        for (source, name) in tests {
            let mut lexer = Lexer::new(source);
            let expected_token = TokenKind::Identifier { name: name.into() };
            assert_eq!(lexer.next(), Ok(Some(expected_token)));
        }
    }

    #[test]
    fn test_valid_keywords() {
        let tests = [
            ("view", TokenKind::View),
            ("script", TokenKind::Script),
            ("style", TokenKind::Style),
            ("let", TokenKind::Let),
            ("mut", TokenKind::Mut),
        ];

        for (source, token) in tests {
            let mut lexer = Lexer::new(source);
            assert_eq!(lexer.next(), Ok(Some(token)));
        }
    }
}
