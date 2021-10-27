use std::fmt;

use logos::Logos;

#[derive(Debug, Copy, Clone, PartialEq, Logos)]
pub enum TokenKind {
    #[regex("[ \n]+")]
    Whitespace,

    #[token(";")]
    Semicolon,

    #[token(":")]
    Colon,

    #[token(",")]
    Comma,

    #[regex("//.*")]
    Comment,

    #[regex("[a-zA-Z_$][a-zA-Z0-9_$]*")]
    Ident,

    #[regex("[0-9]+")]
    LiteralInteger,

    #[regex("\"(\\\\.|[^\"\\\\])*\"")]
    LiteralString,

    #[token("true")]
    LiteralTrue,

    #[token("false")]
    LiteralFalse,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token("/")]
    Slash,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("=")]
    Equals,

    // Keywords
    #[token("func")]
    Func,

    #[token("comp")]
    Comp,

    #[token("let")]
    Let,

    #[error]
    Error,
}

impl TokenKind {
    pub fn is_trivia(self) -> bool {
        matches!(self, TokenKind::Whitespace | TokenKind::Comment)
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::LiteralFalse => "false",
            Self::LiteralTrue => "true",
            Self::Whitespace => "whitespace",
            Self::Comma => "`,`",
            Self::Colon => "`:`",
            Self::Semicolon => "`;`",
            Self::Func => "func",
            Self::Comp => "Comp",
            Self::Let => "let",
            Self::Ident => "identifier",
            Self::LiteralInteger => "number",
            Self::LiteralString => "string literal",
            Self::Plus => "`+`",
            Self::Minus => "`-`",
            Self::Star => "`*`",
            Self::Slash => "`/`",
            Self::Equals => "`=`",
            Self::LParen => "`(`",
            Self::RParen => "`)`",
            Self::LBrace => "`{`",
            Self::RBrace => "`}`",
            Self::LBracket => "`[`",
            Self::RBracket => "`]`",
            Self::Comment => "comment",
            Self::Error => "an unrecognized token",
        })
    }
}
