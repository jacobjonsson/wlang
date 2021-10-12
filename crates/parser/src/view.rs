use ast::ViewNode;

use crate::{error::ParserError, Parser, ParserResult};

impl<'a> Parser<'a> {
    pub(crate) fn parse_view(&mut self) -> ParserResult<Vec<ViewNode>> {
        // Skip the leading {
        self.bump();

        self.skip_whitespace();

        let nodes = Vec::new();

        loop {
            let character = match self.current_character() {
                Some(c) => c,
                None => return Err(ParserError::UnterminatedBlock),
            };

            if character == '}' {
                self.bump();
                break;
            }

            // Parse html-like comment
            if character == '<' && self.next_character() == Some('!') {
                todo!("html-like comments are not implemented yet")
            }

            if character == '<' {
                todo!("parse html element")
            }

            todo!("parse text")
        }

        Ok(nodes)
    }
}
