use lexer::TokenKind;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u16)]
pub enum SyntaxKind {
    Root,

    Tombstone,
    Error,

    // Trivia
    Whitespace,
    Comment,

    // Tokens
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
    EqualsEquals,
    BangEquals,
    Percent,
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
    Literal,
    Integer,
    String,
    FuncKeyword,
    CompKeyword,
    LetKeyword,
    StateKeyword,
    MutKeyword,
    EffectKeyword,
    OnMountKeyword,
    OnUpdateKeyword,
    OnDestroyKeyword,

    // Nodes
    BlockStmt,
    FunctionDecl,
    InfixExpr,
    ParamList,
    ParenExpr,
    PrefixExpr,
    VariableDef,
    VariableRef,

    // This variant is only used to guard against memory errors
    // when converting from and into a u16.
    #[doc(hidden)]
    __LAST,
}

impl From<u16> for SyntaxKind {
    #[inline]
    fn from(d: u16) -> SyntaxKind {
        assert!(d <= (SyntaxKind::__LAST as u16));
        unsafe { std::mem::transmute::<u16, SyntaxKind>(d) }
    }
}

impl From<SyntaxKind> for u16 {
    #[inline]
    fn from(k: SyntaxKind) -> Self {
        k as u16
    }
}

impl From<TokenKind> for SyntaxKind {
    fn from(token_kind: TokenKind) -> Self {
        match token_kind {
            TokenKind::AmpersandAmpersand => Self::AmpersandAmpersand,
            TokenKind::Bang => Self::Bang,
            TokenKind::BangEquals => Self::BangEquals,
            TokenKind::BarBar => Self::BarBar,
            TokenKind::Colon => Self::Colon,
            TokenKind::Comma => Self::Comma,
            TokenKind::Comment => Self::Comment,
            TokenKind::CompKeyword => Self::CompKeyword,
            TokenKind::EffectKeyword => Self::EffectKeyword,
            TokenKind::Equals => Self::Equals,
            TokenKind::EqualsEquals => Self::EqualsEquals,
            TokenKind::Error => Self::Error,
            TokenKind::False => Self::False,
            TokenKind::FuncKeyword => Self::FuncKeyword,
            TokenKind::GreaterThan => Self::GreaterThan,
            TokenKind::GreaterThanEqual => Self::GreaterThanEqual,
            TokenKind::Ident => Self::Ident,
            TokenKind::Integer => Self::Integer,
            TokenKind::LBrace => Self::LBrace,
            TokenKind::LBracket => Self::LBracket,
            TokenKind::LessThan => Self::LessThan,
            TokenKind::LessThanEqual => Self::LessThanEqual,
            TokenKind::LetKeyword => Self::LetKeyword,
            TokenKind::LParen => Self::LParen,
            TokenKind::Minus => Self::Minus,
            TokenKind::MutKeyword => Self::MutKeyword,
            TokenKind::OnDestroyKeyword => Self::OnDestroyKeyword,
            TokenKind::OnMountKeyword => Self::OnMountKeyword,
            TokenKind::OnUpdateKeyword => Self::OnUpdateKeyword,
            TokenKind::Percent => Self::Percent,
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
        SyntaxKind::from(raw.0)
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.into())
    }
}
