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

    #[token("!=")]
    BangEquals,

    #[token("!")]
    Bang,

    #[token(">")]
    GreaterThan,

    #[token(">=")]
    GreaterThanEqual,

    #[token("<")]
    LessThan,

    #[token("<=")]
    LessThanEqual,

    #[token("&&")]
    AmpersandAmpersand,

    #[token("||")]
    BarBar,

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
            Self::AmpersandAmpersand => "`&&`",
            Self::Bang => "`!`",
            Self::BangEquals => "`!=`",
            Self::BarBar => "`||`",
            Self::Colon => "`:`",
            Self::Comma => "`,`",
            Self::Comment => "comment",
            Self::CompKeyword => "Comp",
            Self::EffectKeyword => "effect",
            Self::Equals => "`=`",
            Self::Error => "an unrecognized token",
            Self::False => "false",
            Self::FuncKeyword => "func",
            Self::GreaterThan => "`>`",
            Self::GreaterThanEqual => "`>=`",
            Self::Ident => "identifier",
            Self::Integer => "number",
            Self::LBrace => "`{`",
            Self::LBracket => "`[`",
            Self::LessThan => "`<`",
            Self::LessThanEqual => "`<=`",
            Self::LetKeyword => "let",
            Self::LParen => "`(`",
            Self::Minus => "`-`",
            Self::MutKeyword => "mut",
            Self::OnDestroyKeyword => "onDestroy",
            Self::OnMountKeyword => "onMount",
            Self::OnUpdateKeyword => "onUpdate",
            Self::Plus => "`+`",
            Self::RBrace => "`}`",
            Self::RBracket => "`]`",
            Self::RParen => "`)`",
            Self::Semicolon => "`;`",
            Self::Slash => "`/`",
            Self::Star => "`*`",
            Self::StateKeyword => "state",
            Self::String => "string",
            Self::True => "true",
            Self::Whitespace => "whitespace",
        })
    }
}
