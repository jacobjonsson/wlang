mod cursor;
mod error;
mod identifier;
mod number;
mod string;
mod whitespace;

use std::thread::current;

pub use error::LexerError;
use identifier::is_identifier_start;
use token::TokenKind;

pub type LexerResult<T> = Result<T, LexerError>;

pub struct Lexer<'a> {
    /// The original input
    input: &'a str,

    /// The current index
    index: usize,

    /// The last position in the source
    last_position: usize,

    /// The vector of characters
    characters: Vec<(usize, char)>,

    /// The current token
    pub token: TokenKind,

    /// The start position of the token
    pub token_start: usize,

    /// The end position of the token
    pub token_end: usize,

    /// The string value of the token
    pub token_text: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        let characters: Vec<(usize, char)> = input.char_indices().collect();
        let last_position = characters
            .last()
            .map(|(idx, char)| idx + char.len_utf8())
            .unwrap();

        Lexer {
            input,
            index: 0,
            last_position,
            characters,
            token: TokenKind::EndOfFile,
            token_start: 0,
            token_end: 0,
            token_text: "",
        }
    }

    pub fn expect(&mut self, token: TokenKind) -> LexerResult<()> {
        if self.token != token {
            return Err(LexerError::UnexpectedToken);
        }

        self.next()
    }

    pub fn slice(&self, start: usize, end: usize) -> &'a str {
        &self.input[start..end]
    }

    /// Scans the source code in a global context
    pub fn scan_global(&mut self) -> LexerResult<()> {
        self.skip_whitespace()?;

        let character = match self.current_character() {
            Some(c) => c,
            None => {
                self.token = TokenKind::EndOfFile;
                return Ok(());
            }
        };

        match character {
            c if is_identifier_start(c) => {
                let identifier = self.scan_identifier()?;

                self.token = match identifier {
                    "script" => TokenKind::Script,
                    "view" => TokenKind::View,
                    "style" => TokenKind::Style,
                    _ => return Err(LexerError::UnexpectedToken),
                }
            }

            '{' => {
                self.index += 1;
                self.token = TokenKind::OpenBrace;
            }

            '}' => {
                self.index += 1;
                self.token = TokenKind::CloseBrace;
            }

            _ => return Err(LexerError::UnexpectedToken),
        };

        Ok(())
    }

    pub fn next(&mut self) -> LexerResult<()> {
        self.skip_whitespace()?;

        let current = match self.current_character() {
            Some(c) => c,
            None => {
                self.token = TokenKind::EndOfFile;
                return Ok(());
            }
        };

        let token = match current {
            c if is_identifier_start(c) => {
                self.token_text = self.scan_identifier()?;
                TokenKind::Identifier
            }

            '"' => self.scan_string()?,

            '0'..='9' => self.scan_number()?,

            '(' => {
                self.index += 1;
                TokenKind::OpenParen
            }

            ')' => {
                self.index += 1;
                TokenKind::CloseParen
            }

            '{' => {
                self.index += 1;
                TokenKind::OpenBrace
            }

            '}' => {
                self.index += 1;
                TokenKind::CloseBrace
            }

            '[' => {
                self.index += 1;
                TokenKind::OpenBracket
            }

            ']' => {
                self.index += 1;
                TokenKind::CloseBracket
            }

            '+' => {
                self.index += 1;
                if self.current_character() == Some('=') {
                    self.index += 1;
                    TokenKind::PlusEqual
                } else {
                    TokenKind::Plus
                }
            }

            '-' => {
                self.index += 1;
                if self.current_character() == Some('=') {
                    self.index += 1;
                    TokenKind::MinusEqual
                } else {
                    TokenKind::Minus
                }
            }

            '*' => {
                self.index += 1;
                if self.current_character() == Some('=') {
                    self.index += 1;
                    TokenKind::StarEqual
                } else {
                    TokenKind::Star
                }
            }

            '/' => {
                self.index += 1;
                if self.current_character() == Some('=') {
                    self.index += 1;
                    TokenKind::SlashEqual
                } else {
                    TokenKind::Slash
                }
            }

            '%' => {
                self.index += 1;
                if self.current_character() == Some('=') {
                    self.index += 1;
                    TokenKind::PercentEqual
                } else {
                    TokenKind::Percent
                }
            }

            '.' => {
                self.index += 1;
                TokenKind::Dot
            }

            ':' => {
                self.index += 1;
                if self.current_character() == Some(':') {
                    self.index += 1;
                    TokenKind::ColonColon
                } else {
                    TokenKind::Colon
                }
            }

            '=' => {
                self.index += 1;
                if self.current_character() == Some('=') {
                    self.index += 1;
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                }
            }

            _ => return Err(LexerError::UnexpectedToken),
        };

        self.token = token;

        Ok(())
    }

    fn current_character(&self) -> Option<char> {
        match self.characters.get(self.index) {
            Some((_, c)) => Some(*c),
            None => None,
        }
    }

    fn current_position(&self) -> usize {
        match self.characters.get(self.index) {
            Some((i, _)) => *i,
            None => self.last_position,
        }
    }

    fn next_character(&self) -> Option<char> {
        match self.characters.get(self.index + 1) {
            Some((_, c)) => Some(*c),
            None => None,
        }
    }

    fn next_position(&self) -> usize {
        match self.characters.get(self.index + 1) {
            Some((i, _)) => *i,
            None => self.last_position,
        }
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
            assert_eq!(lexer.next(), Ok(()));
            assert_eq!(lexer.token, token);
        }
    }
}
