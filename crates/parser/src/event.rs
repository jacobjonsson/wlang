use syntax::SyntaxKind;

use crate::parser::ParseError;

#[derive(Debug, PartialEq)]
pub(super) enum Event {
    Start {
        kind: SyntaxKind,
        forward_parent: Option<usize>,
    },
    Finish,
    AddToken,
    Error(ParseError),
}

impl Event {
    pub(crate) fn tombstone() -> Event {
        Event::Start {
            forward_parent: None,
            kind: SyntaxKind::Tombstone,
        }
    }
}
