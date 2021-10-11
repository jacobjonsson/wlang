use std::fmt;

#[derive(PartialEq, PartialOrd, Debug)]
pub enum TokenKind {
    Identifier { name: String },
    String { value: String },
    Integer { value: String },
    Float { value: String },
    Bool { value: bool },

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
    Plus,    // +
    Minus,   // -
    Star,    // *
    Slash,   // /
    Percent, // %

    // Assignments
    Equal,        // =,
    PlusEqual,    // +=,
    MinusEqual,   // -=,
    StarEqual,    // *=,
    PercentEqual, // %=,
    SlashEqual,   // /=

    // Comparison
    EqualEqual,         // ==
    NotEqual,           // !=
    AmpersandAmpersand, // &&
    BarBar,             // ||
    Less,               // <
    LessEqual,          // <=
    Greater,            // >
    GreaterEqual,       // >=

    // Keywords
    Let,
    Mut,
    Fn,
    View,
    Script,
    Style,
    Use,
    Pub,
    Async,
    Await,
}

impl TokenKind {
    pub fn from_string(word: String) -> TokenKind {
        match word.as_str() {
            "let" => TokenKind::Let,
            "fn" => TokenKind::Fn,
            "view" => TokenKind::View,
            "script" => TokenKind::Script,
            "style" => TokenKind::Style,
            "pub" => TokenKind::Pub,
            "use" => TokenKind::Use,
            "async" => TokenKind::Async,
            "mut" => TokenKind::Mut,
            "true" => TokenKind::Bool { value: true },
            "false" => TokenKind::Bool { value: false },
            _ => TokenKind::Identifier { name: word },
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TokenKind::SlashEqual => "/=",
            TokenKind::String { value } => value,
            TokenKind::Integer { value } => value,
            TokenKind::Float { value } => value,
            TokenKind::Identifier { name } => name,
            TokenKind::Bool { value } => {
                if *value {
                    "true"
                } else {
                    "false"
                }
            }
            TokenKind::OpenParen => "(",
            TokenKind::CloseParen => ")",
            TokenKind::OpenBracket => "[",
            TokenKind::CloseBracket => "]",
            TokenKind::OpenBrace => "{",
            TokenKind::CloseBrace => "}",
            TokenKind::Semicolon => ";",
            TokenKind::Colon => ":",
            TokenKind::ColonColon => "::",
            TokenKind::Dot => ".",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Star => "*",
            TokenKind::Slash => "/",
            TokenKind::Percent => "%",
            TokenKind::AmpersandAmpersand => "&&",
            TokenKind::BarBar => "||",
            TokenKind::Less => "<",
            TokenKind::LessEqual => "<=",
            TokenKind::Greater => ">",
            TokenKind::GreaterEqual => ">=",
            TokenKind::Equal => "=",
            TokenKind::EqualEqual => "==",
            TokenKind::NotEqual => "!=",
            TokenKind::Let => "let",
            TokenKind::Mut => "mut",
            TokenKind::Fn => "fn",
            TokenKind::View => "view",
            TokenKind::Script => "script",
            TokenKind::Style => "style",
            TokenKind::Use => "use",
            TokenKind::Pub => "pub",
            TokenKind::Async => "async",
            TokenKind::Await => "await",
            TokenKind::PlusEqual => "+=",
            TokenKind::MinusEqual => "-=",
            TokenKind::StarEqual => "*=",
            TokenKind::PercentEqual => "%=",
        };

        write!(f, "\"{}\"", s)
    }
}
