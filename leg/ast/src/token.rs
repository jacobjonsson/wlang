use syntax::{syntax_kind::SyntaxKind, SyntaxToken};

#[derive(Debug, Clone, PartialEq)]
pub struct Integer(SyntaxToken);

impl Integer {
    pub fn cast(syntax: SyntaxToken) -> Option<Integer> {
        if syntax.kind() == SyntaxKind::Integer {
            Some(Integer(syntax))
        } else {
            None
        }
    }

    pub fn parse(&self) -> Option<u64> {
        self.0.text().parse().ok()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct String(SyntaxToken);

impl String {
    pub fn cast(syntax: SyntaxToken) -> Option<String> {
        if syntax.kind() == SyntaxKind::String {
            Some(String(syntax))
        } else {
            None
        }
    }

    pub fn value(&self) -> &str {
        self.0.text()
    }
}
