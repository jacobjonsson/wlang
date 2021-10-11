mod cursor;
mod error;

use cursor::Cursor;
pub use error::LexerError;
use token::TokenKind;

pub type LexerResult<T> = Result<T, LexerError>;

pub struct Lexer<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &str) -> Lexer {
        Lexer {
            cursor: Cursor::new(source),
        }
    }

    pub fn next(&mut self) -> Result<Option<TokenKind>, LexerError> {
        let current = match self.cursor.current() {
            Some(c) => c,
            None => return Ok(None),
        };

        let token = match current {
            '+' => {
                self.cursor.bump();
                if self.cursor.current() == Some('=') {
                    self.cursor.bump();
                    TokenKind::PlusEqual
                } else {
                    TokenKind::Plus
                }
            }

            '%' => {
                self.cursor.bump();
                if self.cursor.current() == Some('=') {
                    self.cursor.bump();
                    TokenKind::PercentEqual
                } else {
                    TokenKind::Percent
                }
            }

            _ => return Err(LexerError::UnexpectedToken),
        };

        Ok(Some(token))
    }
}

#[cfg(test)]
mod tests {
    use token::TokenKind;

    use crate::Lexer;

    #[test]
    fn test_punctuation() {
        let tests = vec![
            ("+", TokenKind::Plus),
            ("+=", TokenKind::PlusEqual),
            ("%", TokenKind::Percent),
            ("%=", TokenKind::PercentEqual),
        ];

        for (source, token) in tests {
            let mut lexer = Lexer::new(source);
            assert_eq!(lexer.next(), Ok(Some(token)))
        }
    }
}
