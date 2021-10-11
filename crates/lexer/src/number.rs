use token::TokenKind;

use crate::{identifier::is_identifier_start, Lexer, LexerError, LexerResult};

impl<'a> Lexer<'a> {
    pub(crate) fn scan_number(&mut self) -> LexerResult<TokenKind> {
        let start = self.cursor.current_position();
        let mut is_floating_point = false;
        loop {
            let character = match self.cursor.current() {
                Some(c) => c,
                None => break,
            };

            if matches!(character, '0'..='9') {
                self.cursor.bump();
                continue;
            }

            if character == '_' {
                self.cursor.bump();
                continue;
            }

            if character == '.' {
                if is_floating_point {
                    return Err(LexerError::MultipleDotsInNumber);
                }

                is_floating_point = true;
                self.cursor.bump();
                continue;
            }

            if is_identifier_start(character) {
                return Err(LexerError::IdentifierAfterNumber);
            }

            break;
        }

        let end = self.cursor.current_position();
        let text = self.cursor.slice(start, end).replace("_", "");
        if is_floating_point {
            Ok(TokenKind::Float { value: text })
        } else {
            Ok(TokenKind::Integer { value: text })
        }
    }
}

#[cfg(test)]
mod tests {
    use token::TokenKind;

    use crate::{Lexer, LexerError};

    #[test]
    fn test_integer() {
        let tests = [("123", "123"), ("1_2_3_4_5", "12345")];

        for (source, value) in tests {
            let mut lexer = Lexer::new(source);
            let expected_token = TokenKind::Integer {
                value: value.into(),
            };
            assert_eq!(lexer.next(), Ok(Some(expected_token)))
        }
    }

    #[test]
    fn test_float() {
        let tests = [("1.23", "1.23"), ("1_2_3_4_5.3", "12345.3")];

        for (source, value) in tests {
            let mut lexer = Lexer::new(source);
            let expected_token = TokenKind::Float {
                value: value.into(),
            };
            assert_eq!(lexer.next(), Ok(Some(expected_token)))
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
