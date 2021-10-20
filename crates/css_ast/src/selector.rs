/// This file contains the AST nodes for css selectors
/// https://www.w3.org/TR/selectors-4/#selector

/// https://www.w3.org/TR/selectors-4/#simple
#[derive(Debug)]
pub enum SimpleSelector {
    Type {
        name: String,
    },
    Universal,
    Attribute {
        name: String,
        matcher: Option<AttributeSelectorMatcher>,
    },
    Class {
        name: String,
    },
    Id {
        name: String,
    },
    PseudoElement {
        name: String,
    },
    PseudoClass {
        name: String,
        args: Vec<()>,
    },
}

#[derive(Debug)]
pub enum AttributeSelectorMatcher {
    /// `=`
    Equals,

    /// `~=`
    TildeEquals,

    /// `|=`
    BarEquals,

    /// `^=`
    CaretEquals,

    /// `$=`
    DollarEquals,

    /// `*=`
    AsteriskEquals,
}

/// https://www.w3.org/TR/selectors-4/#compound
#[derive(Debug)]
pub struct CompoundSelector {
    pub selectors: Vec<SimpleSelector>,

    pub combinator: Option<SelectorCombinator>,
}

#[derive(Debug)]
pub struct ComplexSelector {
    pub selectors: Vec<CompoundSelector>,
}

/// https://www.w3.org/TR/selectors-4/#selector-combinator
#[derive(Debug)]
pub enum SelectorCombinator {
    /// ` `
    Descendant,

    /// `+`
    NextSibling,

    /// `>`
    Child,

    /// `~`
    LaterSibling,
}
