use std::str::CharIndices;

#[derive(PartialEq, Debug)]
pub enum Token<'a> {
    EndOfFile,
    Ident(&'a str),
    Function(&'a str),
    AtKeyword(&'a str),
    Hash {
        value: &'a str,
        id: bool,
    },
    String(&'a str),
    BadString,
    Url(&'a str),
    BadUrl,
    Delim(char),
    Number {
        value: f32,
        int_value: Option<i32>,
    },
    Percentage {
        value: f32,
    },
    Dimension {
        value: f32,
        int_value: Option<i32>,
        unit: &'a str,
    },
    Whitespace,
    CDO,
    CDC,
    Colon,
    Semicolon,
    Comma,
    OpenBracket,
    CloseBracket,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
}

fn is_name_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_name_continue(ch: char) -> bool {
    is_name_start(ch) || matches!(ch, '0'..='9') || ch == '-'
}

fn is_digit(ch: char) -> bool {
    matches!(ch, '0'..='9')
}

pub struct Lexer<'a> {
    source: &'a str,
    iter: CharIndices<'a>,
    current_char: char,
    current_position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer {
        let mut lexer = Lexer {
            source,
            iter: source.char_indices(),
            current_char: '\0',
            current_position: 0,
        };
        // Prime the lexer
        lexer.step();
        lexer
    }

    fn step(&mut self) -> char {
        if let Some((pos, ch)) = self.iter.next() {
            self.current_char = ch;
            self.current_position = pos;
            self.current_char
        } else {
            self.current_position = self.current_position + self.current_char.len_utf8();
            self.current_char = '\0';
            '\0'
        }
    }

    fn slice_from(&self, start: usize) -> &'a str {
        &self.source[start..self.current_position]
    }

    fn peek_nth(&self, n: usize) -> char {
        self.iter.clone().nth(n).map(|(_, ch)| ch).unwrap_or('\0')
    }

    fn peek(&self) -> char {
        self.peek_nth(0)
    }

    fn would_start_a_number(&self) -> bool {
        if is_digit(self.current_char) {
            return true;
        }

        if self.current_char == '.' {
            return is_digit(self.peek());
        }

        if matches!(self.current_char, '+' | '-') {
            if is_digit(self.peek()) {
                return true;
            }

            if self.peek() == '.' && is_digit(self.peek_nth(2)) {
                return true;
            }

            return false;
        }

        return false;
    }

    /// https://www.w3.org/TR/css-syntax-3/#check-if-three-code-points-would-start-an-identifier
    fn would_start_an_identifier(&self) -> bool {
        if is_name_start(self.current_char) {
            return true;
        }

        if self.current_char == '-' {
            let next = self.peek();
            if is_name_start(next) || next == '-' {
                return true;
            }
        }

        return false;
    }

    pub fn next(&mut self) -> Token<'a> {
        match self.current_char {
            '\0' => Token::EndOfFile,

            c if c.is_whitespace() => {
                self.step();
                loop {
                    if self.current_char.is_whitespace() {
                        self.step();
                    } else {
                        break;
                    }
                }
                Token::Whitespace
            }

            '<' => {
                // TODO: Check for CDO token..?
                self.step();
                Token::Delim('<')
            }

            '\\' => {
                panic!("Does not know how to handle escaped names")
            }

            '@' => {
                if self.would_start_an_identifier() {
                    Token::AtKeyword(self.consume_name())
                } else {
                    self.step();
                    Token::Delim('@')
                }
            }

            '#' => {
                self.step();

                if self.current_char == '\\' {
                    panic!("Escaped names is not supported");
                }

                if is_name_continue(self.current_char) {
                    let id = self.would_start_an_identifier();
                    let value = self.consume_name();
                    Token::Hash { value, id }
                } else {
                    Token::Delim('#')
                }
            }

            '.' => {
                if self.would_start_a_number() {
                    self.consume_numeric()
                } else {
                    Token::Delim('.')
                }
            }

            '+' => {
                if self.would_start_a_number() {
                    self.consume_numeric()
                } else {
                    self.step();
                    Token::Delim('+')
                }
            }

            '-' => {
                if self.would_start_a_number() {
                    self.consume_numeric()
                } else {
                    self.step();
                    Token::Delim('-')
                }
            }

            '0'..='9' => self.consume_numeric(),

            '(' => {
                self.step();
                Token::OpenParen
            }

            ')' => {
                self.step();
                Token::CloseParen
            }

            '{' => {
                self.step();
                Token::OpenBrace
            }

            '}' => {
                self.step();
                Token::CloseBrace
            }

            '[' => {
                self.step();
                Token::OpenBracket
            }

            ']' => {
                self.step();
                Token::CloseBracket
            }

            ',' => {
                self.step();
                Token::Comma
            }

            ':' => {
                self.step();
                Token::Colon
            }

            ';' => {
                self.step();
                Token::Semicolon
            }

            '"' => self.consume_string('"'),

            '\'' => self.consume_string('\''),

            c => {
                self.step();
                Token::Delim(c)
            }
        }
    }

    fn consume_numeric(&mut self) -> Token<'a> {
        let (has_sign, sign) = match self.current_char {
            '-' => (true, -1.),
            '+' => (true, 1.),
            _ => (false, 1.),
        };
        if has_sign {
            self.step();
        }

        let mut integral_part: f64 = 0.;
        while let Some(digit) = char::to_digit(self.current_char, 10) {
            integral_part = integral_part * 10. + digit as f64;
            self.step();
        }

        let mut is_integer = true;
        let mut fractional_part: f64 = 0.;
        if self.current_char == '.' && is_digit(self.peek()) {
            is_integer = false;
            self.step(); // Consume .
            let mut factor = 0.1;
            while let Some(digit) = char::to_digit(self.current_char, 10) {
                fractional_part += digit as f64 * factor;
                factor *= 0.1;
                self.step();
            }
        }

        let value = sign * (integral_part + fractional_part);

        let int_value = if is_integer {
            Some(if value >= i32::MAX as f64 {
                i32::MAX
            } else if value <= i32::MIN as f64 {
                i32::MIN
            } else {
                value as i32
            })
        } else {
            None
        };

        if self.current_char == '%' {
            self.step();
            return Token::Percentage {
                value: (value / 100.) as f32,
            };
        }

        if self.would_start_an_identifier() {
            let unit = self.consume_name();
            return Token::Dimension {
                value: value as f32,
                int_value,
                unit,
            };
        }

        return Token::Number {
            value: value as f32,
            int_value,
        };
    }

    fn consume_name(&mut self) -> &'a str {
        let start = self.current_position;
        while is_name_continue(self.current_char) {
            self.step();
        }

        if self.current_char == '\\' {
            panic!("Escaped names is not supported");
        }

        self.slice_from(start)
    }

    fn consume_string(&mut self, ending_char: char) -> Token<'a> {
        self.step();
        let start = self.current_position;
        loop {
            match self.current_char {
                '\n' | '\0' => panic!("Unterminated string literal"),
                ch if ending_char == ch => break,
                '\\' => {
                    if self.peek() == '\n' {
                        self.step();
                        continue;
                    }
                }
                _ => {
                    self.step();
                }
            }
        }
        let text = self.slice_from(start);
        self.step();
        Token::String(text)
    }
}
