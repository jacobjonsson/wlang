use token::TokenKind;

use crate::{identifier::is_identifier_start, Lexer, LexerError, LexerResult};

impl<'a> Lexer<'a> {
    pub(crate) fn scan_number(&mut self) -> LexerResult<TokenKind> {
        let start = self.current_position();
        let mut is_floating_point = false;
        loop {
            let character = match self.current_character() {
                Some(c) => c,
                None => break,
            };

            if matches!(character, '0'..='9') {
                self.index += 1;
                continue;
            }

            if character == '_' {
                self.index += 1;
                continue;
            }

            if character == '.' {
                if is_floating_point {
                    return Err(LexerError::MultipleDotsInNumber);
                }

                is_floating_point = true;
                self.index += 1;
                continue;
            }

            if is_identifier_start(character) {
                return Err(LexerError::IdentifierAfterNumber);
            }

            break;
        }

        let end = self.current_position();
        self.token_text = self.slice(start, end);
        if is_floating_point {
            Ok(TokenKind::Float)
        } else {
            Ok(TokenKind::Integer)
        }
    }
}

#[cfg(test)]
mod tests {
    use token::TokenKind;

    use crate::{Lexer, LexerError};

    #[test]
    fn test_integer() {
        let tests = [("123", "123"), ("1_2_3_4_5", "1_2_3_4_5")];

        for (source, value) in tests {
            let mut lexer = Lexer::new(source);
            assert_eq!(lexer.next(), Ok(()));
            assert_eq!(lexer.token, TokenKind::Integer);
            assert_eq!(lexer.token_text, value);
        }
    }

    #[test]
    fn test_float() {
        let tests = [("1.23", "1.23"), ("1_2_3_4_5.3", "1_2_3_4_5.3")];

        for (source, value) in tests {
            let mut lexer = Lexer::new(source);
            assert_eq!(lexer.next(), Ok(()));
            assert_eq!(lexer.token, TokenKind::Float);
            assert_eq!(lexer.token_text, value);
        }
    }

    #[test]
    fn test_invalid_numbers() {
        let tests = [
            ("123a", LexerError::IdentifierAfterNumber),
            ("1.2.3", LexerError::MultipleDotsInNumber),
        ];

        for (source, error) in tests {
            let mut lexer = Lexer::new(source);
            assert_eq!(lexer.next(), Err(error));
        }
    }
}
