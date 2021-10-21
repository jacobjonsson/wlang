#![allow(dead_code)]
mod token;

use std::str::CharIndices;
pub use token::Token;

pub struct Lexer<'a> {
    /// The original source
    source: &'a str,

    /// The iterator over the chars
    chars: CharIndices<'a>,

    /// The start position of the iterator
    start_position: usize,

    /// The start position of the current char
    /// To get the end position, call self.first_position
    position: usize,

    /// The current line number
    current_line_number: usize,

    /// The start position of the current line
    /// Used to compute the column for the current line.
    current_line_start_position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer<'a> {
        Lexer {
            source,
            chars: source.char_indices(),
            start_position: 0,
            position: 0,
            current_line_number: 1,
            current_line_start_position: 1,
        }
    }

    /// Returns the position of the last element in the iterator
    fn last_position(&self) -> usize {
        self.source
            .char_indices()
            .last()
            .map(|i| i.0 + i.1.len_utf8())
            .unwrap()
    }

    /// Consumes the next character in the input stream
    fn consume_next(&mut self) -> char {
        match self.chars.next() {
            Some((position, ch)) => {
                self.position = position + self.start_position;
                ch
            }

            // End of file, set position to end of the last character
            None => {
                self.position = self.last_position();
                '\0'
            }
        }
    }

    /// Resets the lexer to previously consumed character
    fn reconsume_current(&mut self) {
        self.reset_to(self.position);
    }

    /// Resets the lexer to the given position
    fn reset_to(&mut self, to: usize) {
        self.chars = self.source[to..].char_indices();
        self.start_position = to;
    }

    /// Returns the nth character from the input stream without consuming it.
    fn peek_nth(&self, n: usize) -> char {
        self.chars.clone().nth(n).map(|t| t.1).unwrap_or('\0')
    }

    /// Returns the fist character from the input stream without consuming it.
    fn first(&self) -> char {
        self.chars.clone().nth(0).map(|t| t.1).unwrap_or('\0')
    }

    /// Returns the position of the first character in the input stream without consuming it.
    fn first_position(&self) -> usize {
        self.chars
            .clone()
            .nth(0)
            .map(|t| t.0 + self.start_position)
            .unwrap_or_else(|| self.last_position())
    }

    /// Returns the second character from the input stream without consuming it.
    fn second(&self) -> char {
        self.chars.clone().nth(1).map(|t| t.1).unwrap_or('\0')
    }

    /// Returns the position of the second character in the input stream without consuming it.
    fn second_position(&self) -> usize {
        self.chars
            .clone()
            .nth(1)
            .map(|t| t.0 + self.start_position)
            .unwrap_or_else(|| self.last_position())
    }

    /// Returns the third character from the input stream without consuming it.
    fn third(&self) -> char {
        self.chars.clone().nth(2).map(|t| t.1).unwrap_or('\0')
    }

    /// Returns the position of the third character in the input stream without consuming it.
    fn third_position(&self) -> usize {
        self.chars
            .clone()
            .nth(2)
            .map(|t| t.0 + self.start_position)
            .unwrap_or_else(|| self.last_position())
    }

    /// Returns a slice of the string from the given start position to the start of the next character.
    fn slice_from(&self, start: usize) -> &'a str {
        self.slice_between(start, self.first_position())
    }

    /// Returns a slice of the string from the given start position to the given end position
    fn slice_between(&self, start: usize, end: usize) -> &'a str {
        &self.source[start..end]
    }

    /// Consumes the next token and returns it
    ///
    /// https://drafts.csswg.org/css-syntax/#consume-token
    pub fn next(&mut self) -> Token<'a> {
        self.consume_comments();

        // The order here matches that one of the spec
        // https://drafts.csswg.org/css-syntax/#consume-token
        match self.consume_next() {
            ch if ch.is_whitespace() => {
                while is_whitespace(self.first()) {
                    self.consume_next();
                }

                Token::Whitespace
            }

            '"' => match self.consume_string_token('"') {
                Ok(value) => Token::String(value),
                Err(value) => Token::BadString(value),
            },

            '#' => {
                if is_identifier(self.first()) || self.is_valid_escape(self.first(), self.second())
                {
                    if self.would_start_an_identifier() {
                        Token::IDHash(self.consume_identifier())
                    } else {
                        Token::Hash(self.consume_identifier())
                    }
                } else {
                    Token::Delim('#')
                }
            }

            '\'' => match self.consume_string_token('\'') {
                Ok(value) => Token::String(value),
                Err(value) => Token::BadString(value),
            },

            '(' => Token::OpenParenthesis,

            ')' => Token::CloseParenthesis,

            '+' => {
                if self.would_start_a_number() {
                    self.reconsume_current();
                    self.consume_numeric()
                } else {
                    Token::Delim('+')
                }
            }

            ',' => Token::Comma,

            '-' => {
                if self.would_start_a_number() {
                    self.reconsume_current();
                    self.consume_numeric()
                } else if self.first() == '-' && self.second() == '>' {
                    self.consume_next();
                    self.consume_next();
                    Token::CDC
                } else if self.would_start_an_identifier() {
                    self.reconsume_current();
                    self.consume_ident_like()
                } else {
                    Token::Delim('-')
                }
            }

            '.' => {
                if self.would_start_a_number() {
                    self.reconsume_current();
                    self.consume_numeric()
                } else {
                    Token::Delim('.')
                }
            }

            ':' => Token::Colon,

            ';' => Token::Semicolon,

            '<' => {
                if self.first() == '!' && self.second() == '-' && self.third() == '-' {
                    self.consume_next();
                    self.consume_next();
                    self.consume_next();
                    Token::CDO
                } else {
                    Token::Delim('<')
                }
            }

            '@' => {
                if self.would_start_an_identifier() {
                    Token::AtKeyword(self.consume_identifier())
                } else {
                    Token::Delim('@')
                }
            }

            '[' => Token::OpenSquareBracket,

            '\\' => match self.first() {
                '\0' => Token::Delim('\\'),
                ch if is_newline(ch) => Token::Delim('\\'),
                _ => self.consume_ident_like(),
            },

            ']' => Token::CloseSquareBracket,

            '{' => Token::OpenCurlyBracket,

            '}' => Token::CloseCurlyBracket,

            ch if is_digit(ch) => {
                self.reconsume_current();
                self.consume_numeric()
            }

            ch if is_identifier_start(ch) => {
                self.reconsume_current();
                self.consume_ident_like()
            }

            '\0' => Token::EndOfFile,

            ch => Token::Delim(ch),
        }
    }

    /// Checks if the next three characters would start a number
    ///
    /// https://drafts.csswg.org/css-syntax/#check-if-three-code-points-would-start-a-number
    fn would_start_a_number(&self) -> bool {
        match self.first() {
            '+' | '-' => {
                if is_digit(self.second()) {
                    return true;
                }

                return self.second() == '.' && is_digit(self.third());
            }

            '.' => {
                return is_digit(self.second());
            }

            ch if is_digit(ch) => true,

            _ => false,
        }
    }

    /// Checks if the next three characters would start an identifier
    ///
    /// https://drafts.csswg.org/css-syntax/#check-if-three-code-points-would-start-an-identifier
    fn would_start_an_identifier(&self) -> bool {
        match self.first() {
            '-' => {
                let second = self.second();
                if is_identifier_start(second) || second == '-' {
                    true
                } else {
                    self.is_valid_escape(second, self.third())
                }
            }

            ch if is_identifier_start(ch) => true,

            ch if ch == '\\' => self.is_valid_escape(ch, self.second()),

            _ => false,
        }
    }

    /// Checks if the two given characters make up an escape
    ///
    /// https://drafts.csswg.org/css-syntax/#check-if-two-code-points-are-a-valid-escape
    fn is_valid_escape(&self, first: char, second: char) -> bool {
        match first {
            '\\' => !is_newline(second),
            _ => false,
        }
    }

    /// Consumes an ident like token
    ///
    /// https://drafts.csswg.org/css-syntax/#consume-an-ident-like-token
    fn consume_ident_like(&mut self) -> Token<'a> {
        let value = self.consume_identifier();

        if value.to_lowercase().as_str() == "url" && self.first() == '(' {
            self.consume_next(); // (

            while is_whitespace(self.first()) && is_whitespace(self.second()) {
                self.consume_next();
            }

            match (self.first(), self.second()) {
                ('"' | '\'', _) => return Token::Function(value),

                (ch, '"' | '\'') if is_whitespace(ch) => {
                    return Token::Function(value);
                }

                _ => todo!("Consume url token"),
            }
        } else if self.first() == '(' {
            self.consume_next();
            return Token::Function(value);
        } else {
            return Token::Ident(value);
        }
    }

    /// Consumes a numeric token
    ///
    /// https://drafts.csswg.org/css-syntax/#consume-a-numeric-token
    fn consume_numeric(&mut self) -> Token<'a> {
        let number = self.consume_number();

        if self.would_start_an_identifier() {
            let unit = self.consume_identifier();
            Token::Dimension(number, unit)
        } else if self.first() == '%' {
            Token::Percentage(number)
        } else {
            Token::Number(number)
        }
    }

    /// Consumes a number
    ///
    /// https://drafts.csswg.org/css-syntax/#consume-a-number
    fn consume_number(&mut self) -> &'a str {
        let start = self.position;

        match self.first() {
            '+' | '-' => {
                self.consume_next();
            }
            _ => {}
        };

        while is_digit(self.first()) {
            self.consume_next();
        }

        if self.first() == '.' && is_digit(self.second()) {
            self.consume_next();
            self.consume_next();
            while is_digit(self.first()) {
                self.consume_next();
            }
        }

        // `123e2` or `123e+2` or `123e-2`
        if matches!(self.first(), 'E' | 'e') {
            let second = self.second();
            if matches!(second, '+' | '-') && is_digit(self.third()) {
                self.consume_next();
                self.consume_next();
                self.consume_next();
                while is_digit(self.first()) {
                    self.consume_next();
                }
            } else if is_digit(second) {
                self.consume_next();
                self.consume_next();
                while is_digit(self.first()) {
                    self.consume_next();
                }
            }
        }

        self.slice_from(start)
    }

    /// Consumes an identifier and returns the string slice
    ///
    /// https://drafts.csswg.org/css-syntax/#consume-an-identifier
    fn consume_identifier(&mut self) -> &'a str {
        let start = self.first_position();

        loop {
            match self.consume_next() {
                ch if is_identifier(ch) => continue,

                ch if self.is_valid_escape(ch, self.first()) => {
                    // TODO: This is not entirely correct
                    // See: https://drafts.csswg.org/css-syntax/#consume-an-escaped-code-point
                    self.consume_next();
                }

                _ => {
                    let text = self.slice_between(start, self.position);
                    self.reconsume_current();
                    return text;
                }
            }
        }
    }

    /// If the next two characters indicates a comment,
    /// consume all of the characters until the end of the comment.
    ///
    /// https://drafts.csswg.org/css-syntax/#consume-comments
    fn consume_comments(&mut self) {
        if self.first() == '/' && self.second() == '*' {
            self.consume_next();
            self.consume_next();

            loop {
                if self.first() == '*' && self.second() == '/' {
                    break;
                }

                self.consume_next();
            }

            self.consume_next();
            self.consume_next();
        }
    }

    /// Consume a string token, it assumes the leading char has been consumed.
    /// The ok value is a valid string and the error value is a bad string.
    ///
    /// https://drafts.csswg.org/css-syntax/#consume-a-string-token
    fn consume_string_token(&mut self, ending_char: char) -> Result<&'a str, &'a str> {
        let start = self.first_position();

        loop {
            match self.consume_next() {
                ch if ch == ending_char => {
                    // We use slice_between since we don't want the ending quote
                    // to be included in the value
                    return Ok(self.slice_between(start, self.position));
                }

                '\0' => {
                    return Err(&self.slice_from(start));
                }

                '\\' => match self.first() {
                    '\0' => continue,

                    c if is_newline(c) => {
                        self.current_line_number += 1;
                        self.consume_next();
                    }

                    _ => {
                        self.consume_next();
                        self.consume_next();
                    }
                },

                _ => {}
            }
        }
    }
}

/// https://drafts.csswg.org/css-syntax/#digit
fn is_digit(ch: char) -> bool {
    match ch {
        '0'..='9' => true,
        _ => false,
    }
}

/// https://drafts.csswg.org/css-syntax/#hex-digit
fn is_hex_digit(ch: char) -> bool {
    match ch {
        c if is_digit(c) => true,
        'a'..='f' => true,
        'A'..='F' => true,
        _ => false,
    }
}

/// https://drafts.csswg.org/css-syntax/#uppercase-letter
fn is_uppercase_letter(ch: char) -> bool {
    match ch {
        'A'..='Z' => true,
        _ => false,
    }
}

/// https://drafts.csswg.org/css-syntax/#lowercase-letter
fn is_lowercase_letter(ch: char) -> bool {
    match ch {
        'a'..='z' => true,
        _ => false,
    }
}

/// https://drafts.csswg.org/css-syntax/#letter
fn is_letter(ch: char) -> bool {
    is_uppercase_letter(ch) || is_lowercase_letter(ch)
}

/// https://drafts.csswg.org/css-syntax/#non-ascii-code-point
fn is_non_ascii(ch: char) -> bool {
    ch as u32 >= 0x80
}

/// https://drafts.csswg.org/css-syntax/#identifier-start-code-point
fn is_identifier_start(ch: char) -> bool {
    is_letter(ch) || is_non_ascii(ch) || ch == '_'
}

/// https://drafts.csswg.org/css-syntax/#identifier-code-point
fn is_identifier(ch: char) -> bool {
    is_identifier_start(ch) || is_digit(ch) || ch == '-'
}

/// https://drafts.csswg.org/css-syntax/#non-printable-code-point
fn is_non_printable(ch: char) -> bool {
    match ch {
        '\u{0000}'..='\u{0008}' => true,
        '\u{000B}' => true,
        '\u{000E}'..='\u{001F}' => true,
        '\u{007F}' => true,
        _ => false,
    }
}

/// https://drafts.csswg.org/css-syntax/#newline
fn is_newline(ch: char) -> bool {
    match ch {
        // Line feed
        '\u{000A}' => true,
        // Form feed
        '\u{000C}' => true,
        // Carriage return
        '\u{000D}' => true,
        _ => false,
    }
}

/// https://drafts.csswg.org/css-syntax/#whitespace
fn is_whitespace(ch: char) -> bool {
    match ch {
        c if is_newline(c) => true,
        // Character tabulation
        '\u{0009}' => true,
        // Space
        '\u{0020}' => true,
        _ => false,
    }
}
