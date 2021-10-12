#![allow(unused_assignments)]

mod cursor;
mod error;
mod identifier;
mod script;
mod style;
mod view;
mod whitespace;

use ast::{Component, Script, Style, View};
use error::ParserError;

use crate::identifier::is_identifier_start;

pub type ParserResult<T> = Result<T, ParserError>;

pub struct Parser<'a> {
    source: &'a str,

    characters: Vec<(usize, char)>,
    index: usize,
}

impl<'a> Parser<'a> {
    pub fn new(source: &str) -> Parser {
        Parser {
            source,
            characters: source.char_indices().collect(),
            index: 0,
        }
    }

    /// Parses one component
    pub fn parse(&mut self) -> ParserResult<Component> {
        self.skip_whitespace();

        let mut script = Script { statements: vec![] };
        let mut style = Style { rules: vec![] };
        let mut view = View { children: vec![] };

        let mut seen_view = false;
        let mut seen_script = false;
        let mut seen_style = false;
        let mut seen_props = false;

        while let Some(character) = self.current_character() {
            println!("{:?}", character);

            if !is_identifier_start(character) {
                return Err(ParserError::UnexpectedToken);
            }

            let identifier = self.scan_identifier()?;

            match identifier {
                "props" => {
                    if seen_props {
                        return Err(ParserError::DuplicatedProps)?;
                    }

                    self.bump();

                    seen_props = true;
                    todo!()
                }

                "view" => {
                    if seen_view {
                        return Err(ParserError::DuplicatedView);
                    }

                    self.bump();

                    seen_view = true;
                    view.children = self.parse_view()?;
                }

                "style" => {
                    if seen_style {
                        return Err(ParserError::DuplicatedStyle);
                    }

                    self.bump();

                    seen_style = true;
                    style.rules = self.parse_style()?;
                }

                "script" => {
                    if seen_script {
                        return Err(ParserError::DuplicatedScript);
                    }

                    self.bump();

                    seen_script = true;
                    script.statements = self.parse_script()?;
                }

                _ => return Err(ParserError::UnexpectedToken),
            }
        }

        Ok(Component {
            script,
            style,
            view,
        })
    }
}
