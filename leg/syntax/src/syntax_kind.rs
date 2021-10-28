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
