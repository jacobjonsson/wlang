mod token_cursor;

use css_ast::{
    AtRule, ClassSelector, IdSelector, QualifiedRule, Rule, Selector, SimpleSelector, StyleSheet,
};
use css_lexer::{LexerError, TokenKind};
pub use token_cursor::TokenCursor;

#[derive(Debug)]
pub enum ParserError {
    InvalidId,
    UnexpectedToken,
    UnterminatedSelector,
    LexerError(LexerError),
}

pub type ParserResult<T> = Result<T, ParserError>;

pub struct Parser {
    cursor: TokenCursor,
}

impl Parser {
    pub fn new(cursor: TokenCursor) -> Parser {
        Parser { cursor }
    }

    pub fn parse(mut self) -> ParserResult<StyleSheet> {
        let mut rules = Vec::new();

        loop {
            match self.cursor.current().map(|t| &t.kind) {
                Some(TokenKind::EOF) => break,

                Some(TokenKind::Whitespace) => {
                    self.cursor.increment();
                }

                Some(TokenKind::CDC | TokenKind::CDO) => {
                    self.cursor.increment();
                }

                Some(TokenKind::AtKeyword { .. }) => {
                    let rule = self.parse_at_rule().map(Rule::At)?;
                    rules.push(rule);
                }

                _ => {
                    let rule = self.parse_qualified_rule().map(Rule::Qualified)?;
                    rules.push(rule);
                }
            };
        }

        Ok(StyleSheet { rules })
    }

    fn expect(&mut self, kind: TokenKind) -> ParserResult<()> {
        if self.cursor.current().map(|t| &t.kind) == Some(&kind) {
            self.cursor.increment();
            Ok(())
        } else {
            Err(ParserError::UnexpectedToken)
        }
    }

    /// Expects the token to be an identifier and returns the string
    /// Increments the lexer if the token is an identifier.
    fn expect_identifier(&mut self) -> ParserResult<String> {
        let value = match self.cursor.current().map(|t| &t.kind) {
            Some(TokenKind::Ident { value }) => value.to_string(),

            _ => return Err(ParserError::UnexpectedToken),
        };

        self.cursor.increment();
        Ok(value)
    }

    fn parse_at_rule(&mut self) -> ParserResult<AtRule> {
        todo!()
    }

    fn parse_qualified_rule(&mut self) -> ParserResult<QualifiedRule> {
        let prelude = self.parse_selector_list()?;

        // TODO: Parse declarations

        // Eat up any potential whitespace
        match self.cursor.current().map(|t| &t.kind) {
            Some(TokenKind::Whitespace) => self.cursor.increment(),
            _ => {}
        };

        self.expect(TokenKind::OpenBrace)?;

        self.expect(TokenKind::CloseBrace)?;

        Ok(QualifiedRule {
            prelude,
            declarations: vec![],
        })
    }

    fn parse_selector_list(&mut self) -> ParserResult<Vec<Selector>> {
        let mut selectors_lists = Vec::new();

        loop {
            let selectors = self.parse_simple_selectors()?;

            selectors_lists.push(Selector { selectors });

            // multiple selectors `.a, .b, .c`
            if self.cursor.current().map(|t| &t.kind) == Some(&TokenKind::Comma) {
                self.cursor.increment();
            } else {
                break;
            }
        }

        Ok(selectors_lists)
    }

    fn parse_simple_selectors(&mut self) -> ParserResult<Vec<SimpleSelector>> {
        let mut selectors = Vec::new();

        loop {
            match self.cursor.current().map(|t| &t.kind) {
                // `.my-class`
                Some(TokenKind::Delim { value: '.' }) => {
                    self.cursor.increment();
                    let value = self.expect_identifier()?;
                    selectors.push(SimpleSelector::Class(ClassSelector { value }));
                }

                // `#my-id``
                Some(TokenKind::Hash { is_id, value }) => {
                    if !is_id {
                        return Err(ParserError::InvalidId);
                    }
                    selectors.push(SimpleSelector::Id(IdSelector {
                        value: value.to_string(),
                    }));
                    self.cursor.increment();
                }

                // `[button="title"]`
                Some(TokenKind::OpenBracket) => {
                    todo!("Attribute selector")
                }

                // `:nth-child` or `::first-line`
                Some(TokenKind::Colon) => {
                    if self.cursor.peek().map(|t| &t.kind) == Some(&TokenKind::Colon) {
                        todo!("Pseudo element selector")
                    } else {
                        todo!("Pseudo class selector")
                    }
                }

                // ` `
                // Could both be a descendent combinator or just whitespace
                // before the next comma or open brace
                Some(TokenKind::Whitespace) => {
                    // Means we've reached the end of the selector
                    if matches!(
                        self.cursor.peek().map(|t| &t.kind),
                        Some(&TokenKind::Comma) | Some(&TokenKind::OpenBrace)
                    ) {
                        return Ok(selectors);
                    }

                    self.cursor.increment();
                    selectors.push(SimpleSelector::Descendant);
                }

                // `+`
                Some(TokenKind::Delim { value: '+' }) => {}

                // `~`
                Some(TokenKind::Delim { value: '~' }) => {}

                // `>`
                Some(TokenKind::Delim { value: '>' }) => {}

                Some(TokenKind::Comma) | Some(TokenKind::OpenBrace) => {
                    return Ok(selectors);
                }

                Some(_) | None => return Err(ParserError::UnterminatedSelector),
            }
        }
    }
}
