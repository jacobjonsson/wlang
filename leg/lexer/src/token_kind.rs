use logos::Logos;
use std::fmt;
use syntax::syntax_kind::SyntaxKind;

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

    #[token("%")]
    Percent,

    #[token("=")]
    Equals,

    #[token("==")]
    EqualsEquals,

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

impl From<TokenKind> for SyntaxKind {
    fn from(token: TokenKind) -> Self {
        match token {
            TokenKind::Whitespace => SyntaxKind::Whitespace,
            TokenKind::Semicolon => SyntaxKind::Semicolon,
            TokenKind::Colon => SyntaxKind::Colon,
            TokenKind::Comma => SyntaxKind::Comma,
            TokenKind::Comment => SyntaxKind::Comment,
            TokenKind::Ident => SyntaxKind::Ident,
            TokenKind::Integer => SyntaxKind::Integer,
            TokenKind::String => SyntaxKind::String,
            TokenKind::True => SyntaxKind::True,
            TokenKind::False => SyntaxKind::False,
            TokenKind::LParen => SyntaxKind::LParen,
            TokenKind::RParen => SyntaxKind::RParen,
            TokenKind::LBrace => SyntaxKind::LBrace,
            TokenKind::RBrace => SyntaxKind::RBrace,
            TokenKind::LBracket => SyntaxKind::LBracket,
            TokenKind::RBracket => SyntaxKind::RBracket,
            TokenKind::Slash => SyntaxKind::Slash,
            TokenKind::Plus => SyntaxKind::Plus,
            TokenKind::Minus => SyntaxKind::Minus,
            TokenKind::Star => SyntaxKind::Star,
            TokenKind::Percent => SyntaxKind::Percent,
            TokenKind::Equals => SyntaxKind::Equals,
            TokenKind::EqualsEquals => SyntaxKind::EqualsEquals,
            TokenKind::BangEquals => SyntaxKind::BangEquals,
            TokenKind::Bang => SyntaxKind::Bang,
            TokenKind::GreaterThan => SyntaxKind::GreaterThan,
            TokenKind::GreaterThanEqual => SyntaxKind::GreaterThanEqual,
            TokenKind::LessThan => SyntaxKind::LessThan,
            TokenKind::LessThanEqual => SyntaxKind::LessThanEqual,
            TokenKind::AmpersandAmpersand => SyntaxKind::AmpersandAmpersand,
            TokenKind::BarBar => SyntaxKind::BarBar,
            TokenKind::FuncKeyword => SyntaxKind::FuncKeyword,
            TokenKind::CompKeyword => SyntaxKind::CompKeyword,
            TokenKind::LetKeyword => SyntaxKind::LetKeyword,
            TokenKind::StateKeyword => SyntaxKind::StateKeyword,
            TokenKind::MutKeyword => SyntaxKind::MutKeyword,
            TokenKind::EffectKeyword => SyntaxKind::EffectKeyword,
            TokenKind::OnMountKeyword => SyntaxKind::OnMountKeyword,
            TokenKind::OnUpdateKeyword => SyntaxKind::OnUpdateKeyword,
            TokenKind::OnDestroyKeyword => SyntaxKind::OnDestroyKeyword,
            TokenKind::Error => SyntaxKind::Error,
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::EqualsEquals => "`==`",
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
            Self::Percent => "`%`",
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
