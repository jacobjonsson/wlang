#![allow(dead_code)]
#[cfg(test)]
mod tests;

mod token;

use token::Token;

use crate::token::str_to_keyword;

/// True if `c` is a whitespace
fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        '\u{0009}' // Tab
        | '\u{000B}' // Vertical tab
        | '\u{0020}' // Space
        | '\u{2003}' // Em space
    )
}

/// True if `c` is a line terminator
fn is_line_terminator(c: char) -> bool {
    matches!(
        c,
        '\u{000A}' // Line feed
        | '\u{000D}' // Carriage return
        | '\u{2028}' // Line separator
    )
}

/// True if `c` is valid as a first character of an identifier.
fn is_identifier_star(c: char) -> bool {
    ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || c == '_' || c == '$'
}

/// True if `c` is a valid non-first character of an identifier
fn is_identifier_continue(c: char) -> bool {
    ('a'..='z').contains(&c)
        || ('A'..='Z').contains(&c)
        || ('1'..='9').contains(&c)
        || c == '_'
        || c == '$'
}

pub struct Lexer {
    chars: Vec<char>,
    current: usize,
    char: char,

    pub token: Token,
    pub start: usize,
    pub end: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Lexer {
        let mut lexer = Lexer {
            chars: source.chars().collect(),
            current: 0,
            char: '\0',
            token: Token::EndOfFile,
            start: 0,
            end: 0,
        };

        lexer.step();
        lexer.next();

        lexer
    }

    /// Increments the lexers internal state
    fn step(&mut self) {
        let mut char = self.chars.get(self.current).map(|c| *c);
        if char == None {
            char = Some('\0');
        }

        self.char = char.unwrap();
        self.end = self.current;
        self.current += 1;
    }

    /// Slices from start to end and returns the string
    fn raw(&self) -> String {
        self.chars[self.start..self.end].into_iter().collect()
    }

    pub fn next(&mut self) {
        if is_whitespace(self.char) {
            self.step();
            while is_whitespace(self.char) {
                self.step();
            }
        }

        if is_line_terminator(self.char) {
            self.step();
            while is_line_terminator(self.char) {
                self.step();
            }
        }

        self.start = self.end;
        self.token = Token::EndOfFile;

        match self.char {
            '=' => {
                self.step();
                if self.char == '=' {
                    self.step();
                    self.token = Token::EqualEqual;
                } else {
                    self.token = Token::Equal;
                }
            }

            ';' => {
                self.step();
                self.token = Token::Semicolon;
            }

            '(' => {
                self.step();
                self.token = Token::OpenParen;
            }

            ')' => {
                self.step();
                self.token = Token::CloseParen;
            }

            '{' => {
                self.step();
                self.token = Token::OpenBrace;
            }

            '}' => {
                self.step();
                self.token = Token::CloseBrace;
            }

            '[' => {
                self.step();
                self.token = Token::OpenBracket;
            }

            ']' => {
                self.step();
                self.token = Token::CloseBracket;
            }

            '+' => {
                self.step();
                self.token = Token::Plus;
            }

            '-' => {
                self.step();
                self.token = Token::Minus;
            }

            '/' => {
                self.step();
                self.token = Token::Slash;
            }

            '*' => {
                self.step();
                self.token = Token::Star;
            }

            '>' => {
                self.step();
                if self.char == '=' {
                    self.step();
                    self.token = Token::GreaterEqual;
                } else {
                    self.token = Token::Greater;
                }
            }

            '<' => {
                self.step();
                if self.char == '=' {
                    self.step();
                    self.token = Token::LessEqual;
                } else {
                    self.token = Token::Less;
                }
            }

            '%' => {
                self.step();
                self.token = Token::Percent;
            }

            '|' => {
                self.step();
                if self.char == '|' {
                    self.step();
                    self.token = Token::BarBar;
                } else {
                    self.token = Token::Bar;
                }
            }

            '&' => {
                self.step();
                if self.char == '&' {
                    self.step();
                    self.token = Token::AmpersandAmpersand;
                } else {
                    self.token = Token::Ampersand;
                }
            }

            c if is_identifier_star(c) => {
                self.step();
                while is_identifier_continue(self.char) {
                    self.step();
                }

                let value = self.raw();

                let mut token = str_to_keyword(&value);
                if token == None {
                    token = Some(Token::Identifier(value));
                }
                self.token = token.unwrap();
            }

            c => panic!("Does not know how to handle: {}", c),
        };
    }
}
