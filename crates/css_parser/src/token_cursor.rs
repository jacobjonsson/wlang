use css_lexer::Token;

pub struct TokenCursor {
    tokens: Vec<Token>,
    index: usize,
}

impl TokenCursor {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }
}

impl TokenCursor {
    pub fn current(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    pub fn current_position(&self) -> usize {
        self.index
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index + 1)
    }

    pub fn peek_nth(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.index + n)
    }

    pub fn increment(&mut self) {
        self.index += 1;
    }

    pub fn reset_to(&mut self, to: usize) {
        self.index = to;
    }
}
