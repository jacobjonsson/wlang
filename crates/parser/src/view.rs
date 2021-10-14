use ast::ViewNode;

use crate::{error::ParserError, Parser, ParserResult};

impl<'a> Parser<'a> {
    pub(crate) fn parse_view(&mut self) -> ParserResult<Vec<ViewNode>> {
        assert_eq!(self.consume_char(), '{');

        let nodes = self.parse_nodes()?;

        assert_eq!(self.consume_char(), '}');

        Ok(nodes)
    }

    fn parse_nodes(&mut self) -> ParserResult<Vec<ViewNode>> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();

            if self.current_character() == None {
                return Err(ParserError::UnterminatedBlock);
            }

            if self.current_character() == Some('}') {
                break;
            }

            nodes.push(self.parse_node()?);
        }

        Ok(nodes)
    }

    fn parse_node(&mut self) -> ParserResult<ViewNode> {
        match self.current_character() {
            Some('<') => self.parse_element(),
            Some(_) => Ok(self.parse_text()),
            None => Err(ParserError::UnexpectedToken),
        }
    }

    fn parse_text(&mut self) -> ViewNode {
        ViewNode::Text {
            data: self.consume_while(|c| c != '<'),
        }
    }

    fn parse_element(&mut self) -> ParserResult<ViewNode> {
        assert_eq!(self.consume_char(), '<');
        let tag_name = self.parse_tag_name();
        let attributes = vec![];
        // Is this valid, this could eat up the whitespace leading to the children..?
        self.consume_whitespace();
        if self.current_character() == Some('/') {
            self.bump();
            assert_eq!(self.consume_char(), '>');
            return Ok(ViewNode::Element {
                name: tag_name,
                attributes,
                children: Vec::new(),
            });
        }

        assert_eq!(self.consume_char(), '>');

        let children = Vec::new();
        assert_eq!(self.consume_char(), '<');
        assert_eq!(self.consume_char(), '/');
        assert_eq!(self.parse_tag_name(), tag_name);
        assert_eq!(self.consume_char(), '>');

        Ok(ViewNode::Element {
            name: tag_name,
            attributes,
            children,
        })
    }

    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => true,
            _ => false,
        })
    }
}
