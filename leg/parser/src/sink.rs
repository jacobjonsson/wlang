use super::event::Event;
use crate::{parser::ParseError, Parse};
use lexer::Token;
use rowan::{GreenNodeBuilder, Language};
use std::mem;
use syntax::{syntax_kind::SyntaxKind, WLanguage};

pub(super) struct Sink<'t, 'input> {
    builder: GreenNodeBuilder<'static>,
    tokens: &'t [Token<'input>],
    events: Vec<Event>,
    cursor: usize,
    errors: Vec<ParseError>,
}

impl<'t, 'input> Sink<'t, 'input> {
    pub(super) fn new(tokens: &'t [Token<'input>], events: Vec<Event>) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            tokens,
            events,
            cursor: 0,
            errors: Vec::new(),
        }
    }

    /// Generate the syntax tree with the control of events.
    pub(super) fn finish(mut self) -> Parse {
        let mut forward_parents = Vec::new();

        for idx in 0..self.events.len() {
            match mem::replace(&mut self.events[idx], Event::tombstone()) {
                Event::Start {
                    kind,
                    forward_parent,
                } => {
                    // For events[A, B, C], B is A's forward_parent, C is B's forward_parent,
                    // in the normal control flow, the parent-child relation: `A -> B -> C`,
                    // while with the magic forward_parent, it writes: `C <- B <- A`.

                    forward_parents.push(kind);
                    let mut idx = idx;
                    let mut fp = forward_parent;

                    while let Some(fwd) = fp {
                        idx += fwd;
                        fp = match mem::replace(&mut self.events[idx], Event::tombstone()) {
                            Event::Start {
                                kind,
                                forward_parent,
                            } => {
                                forward_parents.push(kind);
                                forward_parent
                            }

                            _ => unreachable!(),
                        }
                    }

                    for kind in forward_parents.drain(..).rev() {
                        if kind != SyntaxKind::Tombstone {
                            self.builder.start_node(WLanguage::kind_to_raw(kind));
                        }
                    }
                }
                Event::AddToken => self.token(),
                Event::Finish => self.builder.finish_node(),
                Event::Error(error) => self.errors.push(error),
            }

            self.eat_trivia();
        }

        Parse {
            green_node: self.builder.finish(),
            errors: self.errors,
        }
    }

    fn eat_trivia(&mut self) {
        while let Some(token) = self.tokens.get(self.cursor) {
            if !token.kind.is_trivia() {
                break;
            }

            self.token();
        }
    }

    fn token(&mut self) {
        let Token { kind, text, .. } = self.tokens[self.cursor];

        self.builder
            .token(WLanguage::kind_to_raw(kind.into()), text);

        self.cursor += 1;
    }
}
