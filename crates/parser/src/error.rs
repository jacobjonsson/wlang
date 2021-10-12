use lexer::LexerError;

#[derive(PartialEq, PartialOrd, Debug)]
pub enum ParserError {
    UnexpectedToken,
    LexerError(LexerError),
    MissingView,
    UnterminatedBlock,
    DuplicatedView,
    DuplicatedScript,
    DuplicatedStyle,
    DuplicatedProps,
}

impl From<LexerError> for ParserError {
    fn from(error: LexerError) -> Self {
        Self::LexerError(error)
    }
}
