#[derive(PartialEq, Debug)]
pub enum TokenKind {
    EndOfFile,

    Identifier {
        value: String,
    },

    /// `calc(100)`
    Function {
        name: String,
    },

    /// `@`
    AtKeyword {
        value: String,
    },

    /// `#`
    Hash {
        is_id: bool,
        value: String,
    },

    String {
        value: String,
    },

    BadString {
        value: String,
    },

    Url {
        value: String,
    },

    BadUrl {
        value: String,
    },

    Delim {
        value: char,
    },

    /// 100
    Number {
        value: String,
        is_integer: bool,
    },

    /// `100%`
    Percentage {
        value: f64,
    },

    /// `123em`, `123px`, `123rem`
    Dimension {
        value: f64,
        unit: String,
    },

    Whitespace,

    /// `:`
    Colon,

    /// `;`
    Semicolon,

    /// `,``
    Comma,

    /// `[`
    OpenBracket,

    /// `]`
    CloseBracket,

    /// `(`
    OpenParenthesis,

    /// `)`
    CloseParenthesis,

    /// `{`
    OpenBrace,

    /// `}`
    CloseBrace,
}

#[derive(PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
}
