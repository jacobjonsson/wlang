#[derive(PartialEq, PartialOrd, Debug)]
pub enum LexerError {
    UnexpectedToken,
    UnterminatedStringLiteral,
    IdentifierAfterNumber,
    MultipleDotsInNumber,
}
