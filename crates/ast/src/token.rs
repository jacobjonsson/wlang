use syntax::{SyntaxKind, SyntaxToken};

#[derive(Debug)]
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
