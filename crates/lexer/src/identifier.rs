use crate::{token::Token, Lexer};

/// True if c is a considered the start of an identifier
pub fn is_identifier_start(c: char) -> bool {
    ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || c == '_' || c == '$'
}

/// True if c is a considered the continuation of an identifier
pub fn is_identifier_continuation(c: char) -> bool {
    ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || c == '_' || c == '$'
}

impl Lexer {
    pub(crate) fn scan_identifier(&mut self) -> Token {
        loop {
            let character = match self.current_character() {
                Some(c) => c,
                None => break,
            };

            if !is_identifier_continuation(character) {
                break;
            }

            self.index += 1;
        }

        let end = self.current_position();
        let identifier = &self.input[self.token_start..end];
        Token::from_str(identifier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_identifiers() {
        let tests = vec!["a", "_a", "$a", "a_c_$"];

        for test in tests {
            let mut lexer = Lexer::new(test.into());
            lexer.next();
            assert_eq!(lexer.token, Token::Identifier(test.into()))
        }
    }

    #[test]
    fn test_keywords() {
        let tests = vec![("let", Token::Let)];

        for test in tests {
            let mut lexer = Lexer::new(test.0.into());
            lexer.next();
            assert_eq!(lexer.token, test.1);
        }
    }
}
