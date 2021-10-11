mod cursor;
mod error;
mod identifier;
mod string;
mod whitespace;

use cursor::Cursor;
pub use error::LexerError;
use identifier::is_identifier_start;
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
        self.skip_whitespace()?;

        let current = match self.cursor.current() {
            Some(c) => c,
            None => return Ok(None),
        };

        let token = match current {
            c if is_identifier_start(c) => self.scan_identifier()?,

            '"' => self.scan_string()?,

            '(' => {
                self.cursor.bump();
                TokenKind::OpenParen
            }

            ')' => {
                self.cursor.bump();
                TokenKind::CloseParen
            }

            '{' => {
                self.cursor.bump();
                TokenKind::OpenBrace
            }

            '}' => {
                self.cursor.bump();
                TokenKind::CloseBrace
            }

            '[' => {
                self.cursor.bump();
                TokenKind::OpenBracket
            }

            ']' => {
                self.cursor.bump();
                TokenKind::CloseBracket
            }

            '+' => {
                self.cursor.bump();
                if self.cursor.current() == Some('=') {
                    self.cursor.bump();
                    TokenKind::PlusEqual
                } else {
                    TokenKind::Plus
                }
            }

            '-' => {
                self.cursor.bump();
                if self.cursor.current() == Some('=') {
                    self.cursor.bump();
                    TokenKind::MinusEqual
                } else {
                    TokenKind::Minus
                }
            }

            '*' => {
                self.cursor.bump();
                if self.cursor.current() == Some('=') {
                    self.cursor.bump();
                    TokenKind::StarEqual
                } else {
                    TokenKind::Star
                }
            }

            '/' => {
                self.cursor.bump();
                if self.cursor.current() == Some('=') {
                    self.cursor.bump();
                    TokenKind::SlashEqual
                } else {
                    TokenKind::Slash
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

            '.' => {
                self.cursor.bump();
                TokenKind::Dot
            }

            ':' => {
                self.cursor.bump();
                if self.cursor.current() == Some(':') {
                    self.cursor.bump();
                    TokenKind::ColonColon
                } else {
                    TokenKind::Colon
                }
            }

            '=' => {
                self.cursor.bump();
                if self.cursor.current() == Some('=') {
                    self.cursor.bump();
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
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
        let tests = [
            // Delimiters
            ("(", TokenKind::OpenParen),
            (")", TokenKind::CloseParen),
            ("{", TokenKind::OpenBrace),
            ("}", TokenKind::CloseBrace),
            ("[", TokenKind::OpenBracket),
            ("]", TokenKind::CloseBracket),
            (":", TokenKind::Colon),
            ("::", TokenKind::ColonColon),
            (".", TokenKind::Dot),
            // Binary
            ("+", TokenKind::Plus),
            ("-", TokenKind::Minus),
            ("%", TokenKind::Percent),
            ("*", TokenKind::Star),
            ("/", TokenKind::Slash),
            // Assignment
            ("=", TokenKind::Equal),
            ("+=", TokenKind::PlusEqual),
            ("-=", TokenKind::MinusEqual),
            ("%=", TokenKind::PercentEqual),
            ("*=", TokenKind::StarEqual),
            ("/=", TokenKind::SlashEqual),
        ];

        for (source, token) in tests {
            let mut lexer = Lexer::new(source);
            assert_eq!(lexer.next(), Ok(Some(token)))
        }
    }
}
