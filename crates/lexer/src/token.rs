#[derive(PartialEq, PartialOrd, Debug)]
pub enum Token {
    EndOfFile,

    String(String),
    Integer(String),
    Float(String),
    Identifier(String),
    Bool(bool),

    // Delimiter
    OpenParen,    // (
    CloseParen,   // )
    OpenBracket,  // [
    CloseBracket, // ]
    OpenBrace,    // {
    CloseBrace,   // }

    // Binary operators
    Plus,               // +
    Minus,              // -
    Star,               // *
    Slash,              // /
    Percent,            // %
    Ampersand,          // &
    AmpersandAmpersand, // &&
    Bar,                // |
    BarBar,             // ||
    Less,               // <
    LessEqual,          // <=
    Greater,            // >
    GreaterEqual,       // >=

    // Punctuation
    Equal,      // =
    EqualEqual, // ==
    NotEqual,   // !=

    // Keywords
    Let,
    Fn,
}

pub(crate) fn str_to_keyword(word: &str) -> Option<Token> {
    match word {
        "let" => Some(Token::Let),
        "fn" => Some(Token::Fn),
        _ => None,
    }
}
