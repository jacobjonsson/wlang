use crate::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn bump(&mut self) {
        self.index += 1;
    }

    pub(crate) fn current_character(&self) -> Option<char> {
        self.characters.get(self.index).map(|i| i.1)
    }

    pub(crate) fn next_character(&self) -> Option<char> {
        self.characters.get(self.index + 1).map(|i| i.1)
    }

    pub(crate) fn current_position(&self) -> usize {
        self.characters
            .get(self.index)
            .map(|i| i.0)
            .unwrap_or_else(|| self.characters.last().map(|i| i.0).unwrap())
    }

    pub(crate) fn next_position(&self) -> usize {
        self.characters
            .get(self.index + 1)
            .map(|i| i.0)
            .unwrap_or_else(|| self.characters.last().map(|i| i.0).unwrap())
    }

    pub(crate) fn slice(&self, start: usize, end: usize) -> &'a str {
        &self.source[start..end]
    }
}
