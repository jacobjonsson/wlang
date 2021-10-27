use crate::SyntaxKind;

enum NodeOrToken {}

struct NodeData {
    kind: SyntaxKind,
    text_len: usize,
    children: Vec<NodeOrToken>,
}

struct Token {
    kind: SyntaxKind,
}
