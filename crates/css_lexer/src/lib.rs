use cursor::Cursor;

#[derive(Debug, PartialEq)]
pub enum LexerError {
    EOF,
    UnterminatedString,
    UnexpectedChar(char),
    UnterminatedUrl,
}

type LexerResult<T> = Result<T, LexerError>;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    EOF,

    Ident {
        value: String,
    },

    Function {
        value: String,
    },

    AtKeyword {
        value: String,
    },

    Hash {
        value: String,
        is_id: bool,
    },

    String {
        value: String,
    },

    BadString {
        value: String,
    },

    Url {
        value: String,
    },

    BadUrl {
        value: String,
    },

    Delim {
        value: char,
    },

    Number {
        value: String,
    },

    Percent {
        value: String,
    },

    Dimension {
        value: String,
        unit: String,
    },

    Whitespace,

    /// <!--
    CDO,

    /// -->
    CDC,

    /// :
    Colon,

    /// ;
    Semicolon,

    /// ,
    Comma,

    /// [
    OpenBracket,

    /// ]
    CloseBracket,

    /// (
    OpenParen,

    /// )
    CloseParen,

    /// {
    OpenBrace,

    /// }
    CloseBrace,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub start: usize,
    pub end: usize,
    pub kind: TokenKind,
}

pub struct Lexer<C>
where
    C: Cursor<Item = char>,
{
    cursor: C,
}

impl<C> Lexer<C>
where
    C: Cursor<Item = char>,
{
    pub fn new(cursor: C) -> Self {
        Lexer { cursor }
    }

    pub fn next(&mut self) -> LexerResult<Token> {
        self.skip_comments();

        match self.cursor.current() {
            // `    `
            Some(ch) if ch.is_whitespace() => {
                let start = self.cursor.current_position();
                self.skip_whitespace();
                let end = self.cursor.current_position();
                Ok(Token {
                    start,
                    end,
                    kind: TokenKind::Whitespace,
                })
            }

            Some('(') => {
                let start = self.cursor.current_position();
                self.cursor.increment();
                let end = self.cursor.current_position();
                Ok(Token {
                    start,
                    end,
                    kind: TokenKind::OpenParen,
                })
            }

            Some(')') => {
                let start = self.cursor.current_position();
                self.cursor.increment();
                let end = self.cursor.current_position();
                Ok(Token {
                    start,
                    end,
                    kind: TokenKind::CloseParen,
                })
            }

            Some('{') => {
                let start = self.cursor.current_position();
                self.cursor.increment();
                let end = self.cursor.current_position();
                Ok(Token {
                    start,
                    end,
                    kind: TokenKind::OpenBrace,
                })
            }

            Some('}') => {
                let start = self.cursor.current_position();
                self.cursor.increment();
                let end = self.cursor.current_position();
                Ok(Token {
                    start,
                    end,
                    kind: TokenKind::CloseBrace,
                })
            }

            Some('[') => {
                let start = self.cursor.current_position();
                self.cursor.increment();
                let end = self.cursor.current_position();
                Ok(Token {
                    start,
                    end,
                    kind: TokenKind::OpenBracket,
                })
            }

            Some(']') => {
                let start = self.cursor.current_position();
                self.cursor.increment();
                let end = self.cursor.current_position();
                Ok(Token {
                    start,
                    end,
                    kind: TokenKind::CloseBracket,
                })
            }

            Some(',') => {
                let start = self.cursor.current_position();
                self.cursor.increment();
                let end = self.cursor.current_position();
                Ok(Token {
                    start,
                    end,
                    kind: TokenKind::Comma,
                })
            }

            Some(':') => {
                let start = self.cursor.current_position();
                self.cursor.increment();
                let end = self.cursor.current_position();
                Ok(Token {
                    start,
                    end,
                    kind: TokenKind::Colon,
                })
            }

            Some(';') => {
                let start = self.cursor.current_position();
                self.cursor.increment();
                let end = self.cursor.current_position();
                Ok(Token {
                    start,
                    end,
                    kind: TokenKind::Semicolon,
                })
            }

            Some('+') => {
                let start = self.cursor.current_position();
                self.cursor.increment();
                if self.would_start_a_number() {
                    self.cursor.reset_to(start);
                    let kind = self.consume_numeric()?;
                    let end = self.cursor.current_position();
                    Ok(Token { start, end, kind })
                } else {
                    let end = self.cursor.current_position();
                    Ok(Token {
                        start,
                        end,
                        kind: TokenKind::Delim { value: '+' },
                    })
                }
            }

            Some('-') => {
                let start = self.cursor.current_position();
                self.cursor.increment();
                if self.would_start_a_number() {
                    self.cursor.reset_to(start);
                    let kind = self.consume_numeric()?;
                    let end = self.cursor.current_position();
                    Ok(Token { start, end, kind })
                } else if self.cursor.current() == Some('-') && self.cursor.peek() == Some('>') {
                    self.cursor.increment();
                    self.cursor.increment();
                    let end = self.cursor.current_position();
                    Ok(Token {
                        start,
                        end,
                        kind: TokenKind::CDC,
                    })
                } else {
                    let end = self.cursor.current_position();
                    Ok(Token {
                        start,
                        end,
                        kind: TokenKind::Delim { value: '-' },
                    })
                }
            }

            // `/`, `/* ... */`
            Some('/') => {
                if self.cursor.peek() == Some('*') {
                    todo!("Comments is not yet implemented")
                }

                let start = self.cursor.current_position();
                self.cursor.increment();
                let end = self.cursor.current_position();
                Ok(Token {
                    kind: TokenKind::Delim { value: '/' },
                    start,
                    end,
                })
            }

            // `<!--`, `<`
            Some('<') => {
                let start = self.cursor.current_position();
                self.cursor.increment();
                if self.cursor.current() == Some('!')
                    && self.cursor.peek_nth(1) == Some('-')
                    && self.cursor.peek_nth(2) == Some('-')
                {
                    self.cursor.increment();
                    self.cursor.increment();
                    self.cursor.increment();
                    let end = self.cursor.current_position();
                    Ok(Token {
                        start,
                        end,
                        kind: TokenKind::CDO,
                    })
                } else {
                    let end = self.cursor.current_position();
                    Ok(Token {
                        start,
                        end,
                        kind: TokenKind::Delim { value: '<' },
                    })
                }
            }

            Some('@') => {
                let start = self.cursor.current_position();
                self.cursor.increment();
                if self.would_start_an_identifier() {
                    let value = self.consume_name()?;
                    let end = self.cursor.current_position();
                    Ok(Token {
                        start,
                        end,
                        kind: TokenKind::AtKeyword { value },
                    })
                } else {
                    let end = self.cursor.current_position();
                    Ok(Token {
                        start,
                        end,
                        kind: TokenKind::Delim { value: '@' },
                    })
                }
            }

            Some('.') => {
                let start = self.cursor.current_position();
                self.cursor.increment();
                if self.would_start_a_number() {
                    self.cursor.reset_to(start);
                    let kind = self.consume_numeric()?;
                    let end = self.cursor.current_position();
                    Ok(Token { start, end, kind })
                } else {
                    let end = self.cursor.current_position();
                    Ok(Token {
                        start,
                        end,
                        kind: TokenKind::Delim { value: '.' },
                    })
                }
            }

            Some(ch) if ch.is_digit(10) => {
                let start = self.cursor.current_position();
                let kind = self.consume_numeric()?;
                let end = self.cursor.current_position();
                Ok(Token { start, end, kind })
            }

            Some(ch) if is_name_start(ch) => {
                let start = self.cursor.current_position();
                let kind = self.consume_name_like()?;
                let end = self.cursor.current_position();
                Ok(Token { start, end, kind })
            }

            // `"abc"`, `'abc'`
            Some(ch) if ch == '"' || ch == '\'' => {
                let start = self.cursor.current_position();
                self.cursor.increment(); // Skip the leading `"` | `'`
                let value = self.consume_string(ch)?;
                self.cursor.increment();
                let end = self.cursor.current_position();
                Ok(Token {
                    start,
                    end,
                    kind: TokenKind::String { value },
                })
            }

            Some('\\') => {
                println!(
                    "{:?}-{:?}-{:?}",
                    self.cursor.current(),
                    self.cursor.peek(),
                    self.cursor.peek_nth(2)
                );
                todo!("Identifier starting with escape character")
            }

            // `#ffffff`, `#my-id`
            Some('#') => {
                let start = self.cursor.current_position();
                // Skip the leading hash to prime the lexer for identifier check.
                self.cursor.increment();
                if matches!(self.cursor.current(), Some(ch) if is_name_continue(ch)) {
                    let is_id = self.would_start_an_identifier();
                    let value = self.consume_name()?;
                    let end = self.cursor.current_position();
                    Ok(Token {
                        start,
                        end,
                        kind: TokenKind::Hash { value, is_id },
                    })
                } else {
                    let end = self.cursor.current_position();
                    Ok(Token {
                        start,
                        end,
                        kind: TokenKind::Delim { value: '#' },
                    })
                }
            }

            // End of file
            None => Ok(Token {
                kind: TokenKind::EOF,
                start: self.cursor.current_position(),
                end: self.cursor.current_position(),
            }),

            // Catch all for delimiters
            Some(value) => {
                let start = self.cursor.current_position();
                self.cursor.increment();
                let end = self.cursor.current_position();
                Ok(Token {
                    kind: TokenKind::Delim { value },
                    start,
                    end,
                })
            }
        }
    }

    fn would_start_a_number(&self) -> bool {
        match self.cursor.current() {
            Some('+' | '-') => match self.cursor.peek() {
                Some(ch) if ch.is_digit(10) => true,

                Some('.') => {
                    if let Some(ch) = self.cursor.peek_nth(2) {
                        ch.is_digit(10)
                    } else {
                        false
                    }
                }
                _ => false,
            },

            Some('.') => {
                if let Some(ch) = self.cursor.peek() {
                    ch.is_digit(10)
                } else {
                    false
                }
            }

            Some(ch) => ch.is_digit(10),

            _ => false,
        }
    }

    /// https://www.w3.org/TR/css-syntax-3/#check-if-two-code-points-are-a-valid-escape
    fn is_valid_escape(&self) -> bool {
        match self.cursor.current() {
            Some('\\') => return self.cursor.peek() != Some('\n'),
            _ => false,
        }
    }

    fn would_start_an_identifier(&self) -> bool {
        match self.cursor.current() {
            Some(ch) if is_name_start(ch) => true,

            Some('-') => match self.cursor.peek() {
                Some(ch) if is_name_start(ch) => true,
                Some('-') => true,
                _ => false,
            },

            None | Some(_) => false,
        }
    }

    // Keeps incrementing the cursor until the comment has been skipped
    fn skip_comments(&mut self) {
        match (self.cursor.current(), self.cursor.peek()) {
            (Some('/'), Some('*')) => {
                self.cursor.increment();
                self.cursor.increment();

                loop {
                    if self.cursor.current() == Some('*') && self.cursor.peek() == Some('/') {
                        break;
                    }

                    self.cursor.increment();
                }

                self.cursor.increment();
                self.cursor.increment();
            }

            _ => {}
        };
    }

    /// Keeps incrementing the cursor until the current character is not whitespace
    fn skip_whitespace(&mut self) {
        loop {
            if self.cursor.current().is_none() {
                break;
            }

            if self.cursor.current().unwrap().is_whitespace() {
                self.cursor.increment();
                continue;
            }

            break;
        }
    }

    /// https://www.w3.org/TR/css-syntax-3/#consume-a-numeric-token
    fn consume_numeric(&mut self) -> LexerResult<TokenKind> {
        let value = self.consume_number();
        if self.would_start_an_identifier() {
            let unit = self.consume_name()?;
            Ok(TokenKind::Dimension { value, unit })
        } else if self.cursor.current() == Some('%') {
            self.cursor.increment();
            Ok(TokenKind::Percent { value })
        } else {
            Ok(TokenKind::Number { value })
        }
    }

    /// Consume a number
    /// https://www.w3.org/TR/css-syntax-3/#consume-a-number
    fn consume_number(&mut self) -> String {
        let mut number = String::new();

        match self.cursor.current() {
            Some('+') => {
                self.cursor.increment();
                number.push('+');
            }
            Some('-') => {
                self.cursor.increment();
                number.push('-');
            }

            _ => {}
        };

        while let Some(ch) = self.cursor.current() {
            if !ch.is_digit(10) {
                break;
            }
            number.push(ch);
            self.cursor.increment();
        }

        if self.cursor.current() == Some('.')
            && matches!(self.cursor.peek(), Some(ch) if ch.is_digit(10))
        {
            number.push('.');
            self.cursor.increment(); // Skip over the .
            number.push(self.cursor.current().unwrap()); // We can unwrap since we've already checked it.
            self.cursor.increment(); // Skip over the first digit

            while let Some(ch) = self.cursor.current() {
                if !ch.is_digit(10) {
                    break;
                }
                number.push(ch);
                self.cursor.increment();
            }
        }

        // `e12`, `E-123`
        if matches!(self.cursor.current(), Some('E') | Some('e')) {
            if matches!(self.cursor.peek(), Some('-') | Some('+')) {
                if matches!(self.cursor.peek_nth(2), Some(ch) if ch.is_digit(10)) {
                    number.push(self.cursor.current().unwrap());
                    number.push(self.cursor.peek().unwrap());
                    number.push(self.cursor.peek_nth(2).unwrap());
                    self.cursor.increment();
                    self.cursor.increment();
                    self.cursor.increment();
                    while let Some(ch) = self.cursor.current() {
                        if !ch.is_digit(10) {
                            break;
                        }
                        number.push(ch);
                        self.cursor.increment();
                    }
                }
            } else if matches!(self.cursor.peek(), Some(ch) if ch.is_digit(10)) {
                number.push(self.cursor.current().unwrap());
                number.push(self.cursor.peek().unwrap());
                self.cursor.increment();
                self.cursor.increment();
                while let Some(ch) = self.cursor.current() {
                    if !ch.is_digit(10) {
                        break;
                    }
                    number.push(ch);
                    self.cursor.increment();
                }
            }
        }

        number
    }

    fn consume_name_like(&mut self) -> LexerResult<TokenKind> {
        let value = self.consume_name()?;

        // `url(`
        if value.to_lowercase().as_str() == "url" && self.cursor.current() == Some('(') {
            self.cursor.increment();

            while self.cursor.current() == Some(' ') && self.cursor.peek() == Some(' ') {
                self.cursor.increment();
            }

            match (self.cursor.current(), self.cursor.peek()) {
                (Some('"' | '\''), _) => return Ok(TokenKind::Function { value }),
                (Some(' '), Some('"' | '\'')) => return Ok(TokenKind::Function { value }),
                _ => {
                    return Ok(TokenKind::Url {
                        value: self.consume_url()?,
                    })
                }
            };
        }

        if self.cursor.current() == Some('(') {
            self.cursor.increment();
            return Ok(TokenKind::Function { value });
        }

        return Ok(TokenKind::Ident { value });
    }

    fn consume_url(&mut self) -> LexerResult<String> {
        self.skip_whitespace();
        let mut result = String::new();

        loop {
            match self.cursor.current() {
                Some(')') => return Ok(result),
                Some('"' | '\'' | '(') => return Err(LexerError::UnterminatedUrl),
                Some(ch) => result.push(ch),
                None => return Err(LexerError::EOF),
            };

            self.cursor.increment();
        }
    }

    fn consume_string(&mut self, end_char: char) -> LexerResult<String> {
        let mut result = String::new();

        loop {
            match self.cursor.current() {
                None => {
                    return Err(LexerError::UnterminatedString);
                }
                Some('\n') => {
                    return Err(LexerError::UnterminatedString);
                }
                Some('\\') => match self.cursor.peek() {
                    None => {
                        continue;
                    }
                    Some('\n') => {
                        self.cursor.increment();
                    }
                    Some(ch) => {
                        result.push(ch);
                    }
                },
                Some(ch) if ch == end_char => {
                    return Ok(result);
                }
                Some(ch) => {
                    result.push(ch);
                }
            };

            self.cursor.increment();
        }
    }

    fn consume_name(&mut self) -> LexerResult<String> {
        let mut name = String::new();

        loop {
            match self.cursor.current() {
                Some(ch) if is_name_continue(ch) => {
                    name.push(ch);
                }

                Some(ch) if self.is_valid_escape() => {
                    self.cursor.increment();
                    name.push(ch);
                    name.push_str(&self.consume_valid_escape()?);
                }

                _ => {
                    return Ok(name);
                }
            }

            self.cursor.increment();
        }
    }

    /// https://www.w3.org/TR/css-syntax-3/#consume-an-escaped-code-point
    fn consume_valid_escape(&mut self) -> LexerResult<String> {
        match self.cursor.current() {
            Some(ch) if is_hex_digit(ch) => {
                let mut result = String::from(ch);
                self.cursor.increment();
                loop {
                    match self.cursor.current() {
                        Some(ch) if is_hex_digit(ch) => {
                            if result.len() >= 6 {
                                return Ok(result);
                            }

                            result.push(ch);
                        }

                        _ => return Ok(result),
                    }
                }
            }

            Some(ch) => return Ok(ch.into()),

            None => return Err(LexerError::EOF),
        }
    }
}

fn is_uppercase_letter(c: char) -> bool {
    match c {
        'A'..='Z' => true,
        _ => false,
    }
}

fn is_lowercase_letter(c: char) -> bool {
    match c {
        'a'..='z' => true,
        _ => false,
    }
}

fn is_letter(c: char) -> bool {
    is_uppercase_letter(c) || is_lowercase_letter(c)
}

fn is_non_ascii(c: char) -> bool {
    c as u32 >= 0x80
}

fn is_name_start(c: char) -> bool {
    match c {
        c if is_letter(c) || is_non_ascii(c) || c == '_' => true,
        _ => false,
    }
}

fn is_name_continue(c: char) -> bool {
    is_name_start(c)
        || match c {
            c if c.is_digit(10) || c == '-' => true,
            _ => false,
        }
}

fn is_hex_digit(c: char) -> bool {
    match c {
        'a'..='f' => true,
        'A'..='F' => true,
        _ => false,
    }
}
