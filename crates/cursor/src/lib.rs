use std::str::CharIndices;

pub trait Cursor {
    type Item;

    /// Returns the current item without incrementing the lexer
    fn current(&self) -> Option<Self::Item>;

    /// Returns the position in the source of the current char
    fn current_position(&self) -> usize;

    /// Returns the next item without incrementing the lexer
    fn peek(&self) -> Option<Self::Item>;

    /// Returns the nth item without incrementing the lexer
    fn peek_nth(&self, n: usize) -> Option<Self::Item>;

    /// Returns a slice of the source given start and end positions
    fn slice(&self, start: usize, end: usize) -> &str;

    /// Increments the cursor
    fn increment(&mut self);

    /// Resets the cursor to the given position
    fn reset_to(&mut self, to: usize);
}

pub struct StringCursor<'a> {
    iter: CharIndices<'a>,
    source: &'a str,
    start_position: usize,
    last_position: usize,
}

impl<'a> StringCursor<'a> {
    pub fn new(source: &'a str) -> StringCursor {
        StringCursor {
            iter: source.char_indices(),
            source,
            start_position: 0,
            last_position: 0,
        }
    }
}

impl<'a> Cursor for StringCursor<'a> {
    type Item = char;

    fn current(&self) -> Option<char> {
        self.iter.clone().next().map(|i| i.1)
    }

    fn current_position(&self) -> usize {
        self.iter
            .clone()
            .next()
            .map(|i| self.start_position + i.0)
            .unwrap_or(self.last_position)
    }

    fn peek(&self) -> Option<char> {
        self.iter.clone().nth(1).map(|i| i.1)
    }

    fn peek_nth(&self, n: usize) -> Option<char> {
        self.iter.clone().nth(n).map(|i| i.1)
    }

    fn slice(&self, start: usize, end: usize) -> &str {
        assert!(start <= end, "Cannot slice {:?}..{:?}", start, end);
        &self.source[start..end]
    }

    fn increment(&mut self) {
        if let Some((p, c)) = self.iter.next() {
            self.last_position = p + c.len_utf8();
        } else {
            unreachable!("Increment should not be called when current() == None");
        }
    }

    fn reset_to(&mut self, to: usize) {
        let s = &self.source[to..];
        self.iter = s.char_indices();
        self.start_position = to;
    }
}
