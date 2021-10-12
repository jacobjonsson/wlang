use crate::{Parser, ParserResult};

/// True if `c` is considered an identifier start.
pub fn is_identifier_start(c: char) -> bool {
    ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || c == '_' || c == '$'
}

/// True if `c` is considered an identifier start continuation.
pub fn is_identifier_continue(c: char) -> bool {
    ('a'..='z').contains(&c)
        || ('A'..='Z').contains(&c)
        || ('0'..='9').contains(&c)
        || c == '\u{200C}'
        || c == '\u{200D}'
        || c == '_'
        || c == '$'
}

impl<'a> Parser<'a> {
    pub(crate) fn scan_identifier(&mut self) -> ParserResult<&'a str> {
        let start = self.current_position();
        loop {
            let character = match self.current_character() {
                Some(c) => c,
                None => break,
            };

            if !is_identifier_continue(character) {
                break;
            }

            self.index += 1;
        }

        let end = self.current_position();
        Ok(self.slice(start, end))
    }
}
