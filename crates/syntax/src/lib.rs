use lexer::TokenKind;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum SyntaxKind {
    Tombstone,
    Error,
    Root,
    Whitespace,
    Comment,
    Comma,
    Colon,
    Semicolon,
    True,
    False,
    Ident,
    Plus,
    Minus,
    Star,
    Slash,
    Equals,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    InfixExpr,
    Literal,
    Integer,
    String,
    ParenExpr,
    PrefixExpr,
    VariableRef,
    VariableDef,

    // Keywords
    FuncKeyword,
    CompKeyword,
    LetKeyword,
    StateKeyword,
    MutKeyword,
    EffectKeyword,
    OnMountKeyword,
    OnUpdateKeyword,
    OnDestroyKeyword,
}

impl From<TokenKind> for SyntaxKind {
    fn from(token_kind: TokenKind) -> Self {
        match token_kind {
            TokenKind::Whitespace => Self::Whitespace,
            TokenKind::Comma => Self::Comma,
            TokenKind::Semicolon => Self::Semicolon,
            TokenKind::Colon => Self::Colon,
            TokenKind::String => Self::String,
            TokenKind::False => Self::False,
            TokenKind::True => Self::True,
            TokenKind::Integer => Self::Integer,
            TokenKind::Comment => Self::Comment,
            TokenKind::Ident => Self::Ident,
            TokenKind::Plus => Self::Plus,
            TokenKind::Minus => Self::Minus,
            TokenKind::Star => Self::Star,
            TokenKind::Slash => Self::Slash,
            TokenKind::Equals => Self::Equals,
            TokenKind::LParen => Self::LParen,
            TokenKind::RParen => Self::RParen,
            TokenKind::LBrace => Self::LBrace,
            TokenKind::RBrace => Self::RBrace,
            TokenKind::LBracket => Self::LBracket,
            TokenKind::RBracket => Self::RBracket,
            TokenKind::FuncKeyword => Self::FuncKeyword,
            TokenKind::CompKeyword => Self::CompKeyword,
            TokenKind::LetKeyword => Self::LetKeyword,
            TokenKind::StateKeyword => Self::StateKeyword,
            TokenKind::MutKeyword => Self::MutKeyword,
            TokenKind::EffectKeyword => Self::EffectKeyword,
            TokenKind::OnMountKeyword => Self::OnMountKeyword,
            TokenKind::OnUpdateKeyword => Self::OnUpdateKeyword,
            TokenKind::OnDestroyKeyword => Self::OnDestroyKeyword,
            TokenKind::Error => Self::Error,
        }
    }
}

pub type SyntaxNode = rowan::SyntaxNode<WLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<WLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<WLanguage>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<WLanguage>;
pub type SyntaxElementChildren = rowan::SyntaxElementChildren<WLanguage>;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum WLanguage {}

impl rowan::Language for WLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        Self::Kind::from_u16(raw.0).unwrap()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.to_u16().unwrap())
    }
}
