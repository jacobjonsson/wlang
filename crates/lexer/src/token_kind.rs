use logos::Logos;
use std::fmt;

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
    Integer,

    #[regex("\"(\\\\.|[^\"\\\\])*\"")]
    String,

    #[token("true")]
    True,

    #[token("false")]
    False,

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

    #[token("func")]
    FuncKeyword,

    #[token("comp")]
    CompKeyword,

    #[token("let")]
    LetKeyword,

    #[token("state")]
    StateKeyword,

    #[token("mut")]
    MutKeyword,

    #[token("effect")]
    EffectKeyword,

    #[token("onMount")]
    OnMountKeyword,

    #[token("onUpdate")]
    OnUpdateKeyword,

    #[token("onDestroy")]
    OnDestroyKeyword,

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
            Self::False => "false",
            Self::True => "true",
            Self::Whitespace => "whitespace",
            Self::Comma => "`,`",
            Self::Colon => "`:`",
            Self::Semicolon => "`;`",
            Self::Ident => "identifier",
            Self::Integer => "number",
            Self::String => "string literal",
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
            Self::FuncKeyword => "func",
            Self::CompKeyword => "Comp",
            Self::LetKeyword => "let",
            Self::StateKeyword => "state",
            Self::MutKeyword => "mut",
            Self::EffectKeyword => "effect",
            Self::OnMountKeyword => "onMount",
            Self::OnUpdateKeyword => "onUpdate",
            Self::OnDestroyKeyword => "onDestroy",
            Self::Comment => "comment",
            Self::Error => "an unrecognized token",
        })
    }
}
