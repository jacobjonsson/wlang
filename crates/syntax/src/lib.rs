use lexer::TokenKind;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum SyntaxKind {
    Whitespace,
    Ident,
    Number,
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
    Comment,
    Error,
    Root,
    InfixExpr,
    Literal,
    ParenExpr,
    PrefixExpr,
    VariableRef,
    VariableDef,

    // Keywords
    FuncKeyword,
    CompKeyword,
    LetKeyword,
}

impl From<TokenKind> for SyntaxKind {
    fn from(token_kind: TokenKind) -> Self {
        match token_kind {
            TokenKind::Whitespace => Self::Whitespace,
            TokenKind::Comma => todo!(),
            TokenKind::Semicolon => todo!(),
            TokenKind::Colon => todo!(),
            TokenKind::LiteralString => todo!(),
            TokenKind::LiteralFalse => todo!(),
            TokenKind::LiteralTrue => todo!(),
            TokenKind::Comment => Self::Comment,
            TokenKind::Ident => Self::Ident,
            TokenKind::LiteralInteger => Self::Number,
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
            TokenKind::Func => Self::FuncKeyword,
            TokenKind::Comp => Self::CompKeyword,
            TokenKind::Let => Self::LetKeyword,
            TokenKind::Error => Self::Error,
        }
    }
}

pub type SyntaxNode = rowan::SyntaxNode<WLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<WLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<WLanguage>;

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
