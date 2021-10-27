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
    BangEquals,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    AmpersandAmpersand,
    BarBar,
    Bang,
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
            TokenKind::Bang => Self::Bang,
            TokenKind::BangEquals => Self::BangEquals,
            TokenKind::LessThan => Self::LessThan,
            TokenKind::LessThanEqual => Self::LessThanEqual,
            TokenKind::GreaterThan => Self::GreaterThan,
            TokenKind::GreaterThanEqual => Self::GreaterThanEqual,
            TokenKind::AmpersandAmpersand => Self::AmpersandAmpersand,
            TokenKind::BarBar => Self::BarBar,
            TokenKind::Colon => Self::Colon,
            TokenKind::Comma => Self::Comma,
            TokenKind::Comment => Self::Comment,
            TokenKind::CompKeyword => Self::CompKeyword,
            TokenKind::EffectKeyword => Self::EffectKeyword,
            TokenKind::Equals => Self::Equals,
            TokenKind::Error => Self::Error,
            TokenKind::False => Self::False,
            TokenKind::FuncKeyword => Self::FuncKeyword,
            TokenKind::Ident => Self::Ident,
            TokenKind::Integer => Self::Integer,
            TokenKind::LBrace => Self::LBrace,
            TokenKind::LBracket => Self::LBracket,
            TokenKind::LetKeyword => Self::LetKeyword,
            TokenKind::LParen => Self::LParen,
            TokenKind::Minus => Self::Minus,
            TokenKind::MutKeyword => Self::MutKeyword,
            TokenKind::OnDestroyKeyword => Self::OnDestroyKeyword,
            TokenKind::OnMountKeyword => Self::OnMountKeyword,
            TokenKind::OnUpdateKeyword => Self::OnUpdateKeyword,
            TokenKind::Plus => Self::Plus,
            TokenKind::RBrace => Self::RBrace,
            TokenKind::RBracket => Self::RBracket,
            TokenKind::RParen => Self::RParen,
            TokenKind::Semicolon => Self::Semicolon,
            TokenKind::Slash => Self::Slash,
            TokenKind::Star => Self::Star,
            TokenKind::StateKeyword => Self::StateKeyword,
            TokenKind::String => Self::String,
            TokenKind::True => Self::True,
            TokenKind::Whitespace => Self::Whitespace,
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
