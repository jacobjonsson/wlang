use crate::{TextValue, Value};

#[derive(Debug)]
pub struct Selector {
    pub selectors: Vec<SimpleSelector>,
}

#[derive(Debug)]
pub enum SimpleSelector {
    // `*`
    Universal,

    // `#my-id`
    Id(IdSelector),

    // `button`
    Type(TypeSelector),

    // `.my-class`
    Class(ClassSelector),

    // `[name="hello"]`
    Attribute(AttributeSelector),

    // :last-child
    PseudoClass(PseudoClassSelector),

    // ::first-line
    PseudoElement(PseudoElementSelector),

    /// https://www.w3.org/TR/selectors-4/#combinators
    /// Combinators

    /// ` `
    Descendant,
    /// `+`
    NextSibling,
    /// `>`
    Child,
    /// `~`
    LaterSibling,
}

#[derive(Debug)]
pub struct IdSelector {
    pub value: String,
}

#[derive(Debug)]
pub struct TypeSelector {
    pub value: String,
}

#[derive(Debug)]
pub struct ClassSelector {
    pub value: String,
}

#[derive(Debug)]
pub struct AttributeSelector {
    pub attribute: TextValue,
    // Can only really be `Text` or `String`
    pub value: Value,
    pub matcher: AttributeMatcher,
    // `i` or `s`
    pub modifier: char,
}

#[derive(Debug)]
pub enum AttributeMatcher {
    /// `=`
    Equals,
    /// `~=`
    Tilde,
    /// `|=`
    Bar,
    /// `^=`
    Caret,
    /// `$=`
    Dollar,
    /// `*=`
    Asterisk,
}

#[derive(Debug)]
pub struct PseudoClassSelector {
    pub value: String,
    pub args: Option<Vec<Value>>,
}

#[derive(Debug)]
pub struct PseudoElementSelector {
    pub value: String,
}
