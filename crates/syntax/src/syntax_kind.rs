#[repr(u16)]
#[derive(Debug, PartialEq, Clone)]
pub enum SyntaxKind {
    #[doc(hidden)]
    Tombstone,
    #[doc(hidden)]
    EndOfFile,

    // --- Tokens ---
    /// An error token
    Error,
    /// ` `
    Whitespace,
    /// `//...`
    Comment,
    /// `(`
    LeftParen,
    /// `)`
    RightParen,
    /// `{`
    LeftBrace,
    /// `}`
    RightBrace,
    /// `[`
    LeftBracket,
    /// `]`
    RightBracket,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `/`
    Slash,
    /// `*`
    Star,
    /// `%`
    Percent,
    /// `=`
    Equals,
    /// An identifier
    Identifier,
    /// An integer literal
    Integer,
    /// A float literal
    Float,
    /// A string literal
    String,

    // --- Nodes ---

    // Only used to assert when converting to and from u16
    #[doc(hidden)]
    __LAST,
}

impl From<u16> for SyntaxKind {
    #[inline]
    fn from(d: u16) -> Self {
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
