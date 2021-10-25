pub enum TokenKind {
    Identifier,
    String,
    Integer,
    Float,

    // Punctuators
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace
    OpenBracket,
    CloseBracket,
    Colon,
    Semicolon,

    Ampersand,
    AmpersandAmpersand
    Bar,
    BarBar,
    Plus,
    Minus,
    Equal,
    PlusEqual,
    MinusEqual,

    // Keywords
    Function,
    Component,
    State,
    Effect,
    Mount,
    BeforeUpdate,
    AfterUpdate,
}
