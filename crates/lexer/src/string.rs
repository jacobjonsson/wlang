use token::TokenKind;

use crate::{whitespace::is_line_terminator, Lexer, LexerError, LexerResult};

impl<'a> Lexer<'a> {
    pub(crate) fn scan_string(&mut self) -> LexerResult<TokenKind> {
        // Skip the leading quote
        self.index += 1;
        let start = self.current_position();

        loop {
            let character = match self.current_character() {
                Some(c) => c,
                None => return Err(LexerError::UnterminatedStringLiteral),
            };

            if is_line_terminator(character) {
                return Err(LexerError::UnterminatedStringLiteral);
            }

            if character == '"' {
                break;
            }

            self.index += 1;
        }

        // Grab the end position before we skip over the trailing quote.
        let end = self.current_position();
        self.index += 1;

        self.token_text = self.slice(start, end);
        Ok(TokenKind::String)
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
            assert_eq!(lexer.next(), Ok(()));
            assert_eq!(lexer.token, TokenKind::String);
            assert_eq!(lexer.token_text, text);
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
