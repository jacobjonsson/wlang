pub use crate::{selector::*, value::*};

mod selector;
mod value;

#[derive(Debug)]
pub struct StyleSheet {
    pub rules: Vec<Rule>,
}

#[derive(Debug)]
pub enum Rule {
    At(AtRule),
    Qualified(QualifiedRule),
}

#[derive(Debug)]
pub struct AtRule {}

#[derive(Debug)]
pub struct QualifiedRule {
    pub prelude: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug)]
pub struct Declaration {
    pub name: TextValue,
    pub values: Value,
    pub important: bool,
}
