pub enum SyntaxToken {
    String,
    Number,
    FuncKeyword,
    LeftParen,
    RightParen,
}

#[repr(u16)]
pub enum SyntaxKind {
    // Tokens
    StringToken,
    NumberToken,
    // Nodes
}
