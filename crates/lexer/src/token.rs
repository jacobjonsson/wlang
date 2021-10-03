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
    Semicolon,    // ;
    Colon,        // :
    ColonColon,   // ::
    Dot,          // .

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
    Const,
    Fn,
    Component,
    Use,
    Pub,
    Async,
}

pub(crate) fn str_to_keyword(word: &str) -> Option<Token> {
    match word {
        "let" => Some(Token::Let),
        "fn" => Some(Token::Fn),
        "const" => Some(Token::Const),
        "component" => Some(Token::Component),
        "pub" => Some(Token::Pub),
        "use" => Some(Token::Use),
        "async" => Some(Token::Async),
        _ => None,
    }
}
