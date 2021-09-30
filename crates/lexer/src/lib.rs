mod identifier;
mod token;
mod whitespace;

use identifier::is_identifier_start;
use token::Token;

pub struct Lexer {
    input: String,
    characters: Vec<(usize, char)>,
    index: usize,
    last_position: usize,

    pub token: Token,
    pub token_start: usize,
    pub token_end: usize,
}

impl Lexer {
    /// Creates a new lexer instance
    pub fn new(input: String) -> Lexer {
        let characters: Vec<(usize, char)> = input.char_indices().collect();
        let last_position: usize = characters
            .last()
            .map(|(idx, char)| idx + char.len_utf8())
            .expect("Failed to extract the position of the last character");

        Lexer {
            input,
            characters,
            index: 0,
            last_position,
            token: Token::EOF,
            token_start: 0,
            token_end: 0,
        }
    }

    pub fn next(&mut self) {
        self.skip_whitespace();

        self.token_start = self.current_position();
        let character = match self.current_character() {
            Some(c) => c,
            None => return,
        };

        self.token = match character {
            c if is_identifier_start(c) => self.scan_identifier(),
            '0' => todo!(),
            '1'..='9' => todo!(),
            _ => Token::EOF,
        };

        self.token_end = self.current_position();
    }

    /// Returns the current character
    fn current_character(&self) -> Option<char> {
        match self.characters.get(self.index) {
            Some(v) => Some(v.1),
            None => None,
        }
    }

    /// Returns the next character
    fn next_character(&self) -> Option<char> {
        match self.characters.get(self.index + 1) {
            Some(v) => Some(v.1),
            None => None,
        }
    }

    /// Returns the current position in the source
    /// If the current character does not exist the we return
    /// the position of the last character instead.
    fn current_position(&self) -> usize {
        match self.characters.get(self.index) {
            Some(v) => v.0,
            None => self.last_position,
        }
    }

    /// Returns the position in the source of the
    /// previous character.
    fn previous_position(&self) -> usize {
        match self.characters.get(self.index - 1) {
            Some(v) => v.0,
            None => self.last_position,
        }
    }
}
