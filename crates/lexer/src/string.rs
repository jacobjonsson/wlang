use token::TokenKind;

use crate::{whitespace::is_line_terminator, Lexer, LexerError, LexerResult};

impl<'a> Lexer<'a> {
    pub(crate) fn scan_string(&mut self) -> LexerResult<TokenKind> {
        // Skip the leading quote
        self.cursor.bump();
        let start = self.cursor.current_position();

        loop {
            let character = match self.cursor.current() {
                Some(c) => c,
                None => return Err(LexerError::UnterminatedStringLiteral),
            };

            if is_line_terminator(character) {
                return Err(LexerError::UnterminatedStringLiteral);
            }

            if character == '"' {
                break;
            }

            self.cursor.bump();
        }

        // Grab the end position before we skip over the trailing quote.
        let end = self.cursor.current_position();
        self.cursor.bump();

        let text = self.cursor.slice(start, end);
        Ok(TokenKind::String { value: text.into() })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Lexer, LexerError};
    use token::TokenKind;

    #[test]
    fn test_valid_strings() {
        let tests = [("\"Hello world\"", "Hello world")];

        for (source, text) in tests {
            let mut lexer = Lexer::new(source);
            let expected_token = TokenKind::String { value: text.into() };
            assert_eq!(lexer.next(), Ok(Some(expected_token)));
        }
    }

    #[test]
    fn test_invalid_strings() {
        let tests = [("\"hello world")];
        for source in tests {
            let mut lexer = Lexer::new(source);
            assert_eq!(lexer.next(), Err(LexerError::UnterminatedStringLiteral));
        }
    }
}
