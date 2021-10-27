use super::Parser;
use crate::event::Event;
use drop_bomb::DropBomb;
use syntax::SyntaxKind;

pub(crate) struct Marker {
    pos: usize,
    bomb: DropBomb,
}

impl Marker {
    pub(crate) fn new(pos: usize) -> Self {
        Self {
            pos,
            bomb: DropBomb::new("Markers need to be completed"),
        }
    }

    /// Finishes the syntax tree node and assigns `kind` to it,
    /// and mark the create a `CompletedMarker` for possible future
    /// operation like `.precede()` to deal with forward_parent.
    pub(crate) fn complete(mut self, parser: &mut Parser, kind: SyntaxKind) -> CompletedMarker {
        self.bomb.defuse();

        match &mut parser.events[self.pos] {
            Event::Start { kind: slot, .. } => {
                *slot = kind;
            }

            _ => unreachable!(),
        };

        parser.events.push(Event::Finish);
        CompletedMarker { pos: self.pos }
    }
}

pub(crate) struct CompletedMarker {
    pos: usize,
}

impl CompletedMarker {
    /// This method allows to create a new node which starts
    /// *before* the current one. That is, parser could start
    /// node `A`, then complete it, and then after parsing the
    /// whole `A`, decide that it should have started some node
    /// `B` before starting `A`. `precede` allows to do exactly
    /// that. See also docs about
    pub(crate) fn precede(self, parser: &mut Parser) -> Marker {
        let new_m = parser.start();

        match &mut parser.events[self.pos] {
            Event::Start { forward_parent, .. } => {
                *forward_parent = Some(new_m.pos - self.pos);
            }
            _ => unreachable!(),
        }

        new_m
    }
}
