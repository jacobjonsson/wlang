mod green_node;
mod syntax_kind;

#[repr(u16)]
pub enum SyntaxKind {
    // Tokens
    StringToken,
    NumberToken,
    // Nodes
}

#[test]
fn syntax_kind_to_u16() {
    eprintln!("{}", SyntaxKind::NumberToken as u16);
}
