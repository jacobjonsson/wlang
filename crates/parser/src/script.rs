use ast::ScriptStatement;

use crate::{Parser, ParserResult};

impl<'a> Parser<'a> {
    pub(crate) fn parse_script(&mut self) -> ParserResult<Vec<ScriptStatement>> {
        todo!()
    }
}
