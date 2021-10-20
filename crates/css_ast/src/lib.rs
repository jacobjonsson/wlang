pub use crate::{property::*, selector::*, token::*, value::*};

mod property;
mod selector;
mod token;
mod value;

pub struct StyleSheet {
    pub rules: Vec<Rule>,
}

pub enum Rule {
    Qualified(QualifiedRule),
    AtRule(AtRule),
}

pub enum AtRule {}

pub struct QualifiedRule {
    pub selector: Vec<ComplexSelector>,
    pub items: Vec<Property>,
}
