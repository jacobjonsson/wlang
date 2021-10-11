#![allow(dead_code)]

use std::str::CharIndices;

pub struct Cursor<'a> {
    source: &'a str,
    chars: CharIndices<'a>,
    last_post: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(source: &str) -> Cursor {
        Cursor {
            source,
            chars: source.char_indices(),
            last_post: 0,
        }
    }

    pub fn current(&self) -> Option<char> {
        self.chars.clone().nth(0).map(|i| i.1)
    }

    pub fn current_position(&self) -> Option<usize> {
        self.chars.clone().nth(0).map(|i| i.0)
    }

    pub fn peek(&self) -> Option<char> {
        self.chars.clone().nth(1).map(|i| i.1)
    }

    pub fn peek_position(&self) -> Option<usize> {
        self.chars.clone().nth(1).map(|i| i.0)
    }

    // Increments the cursor
    pub fn bump(&mut self) {
        if let Some((i, c)) = self.chars.next() {
            self.last_post = i + c.len_utf8();
        } else {
            unreachable!("bump should not be called when current() = None");
        }
    }

    // Returns a slice of the original string given a start and end position
    pub fn slice(&self, start: usize, end: usize) -> &str {
        &self.source[start..end]
    }
}
