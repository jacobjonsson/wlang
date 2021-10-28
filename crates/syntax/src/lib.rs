mod syntax_kind;

pub use syntax_kind::SyntaxKind;

#[derive(Debug)]
pub struct Token {
    pub kind: SyntaxKind,
    pub len: usize,
}
