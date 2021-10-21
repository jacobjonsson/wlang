#[derive(Debug)]
pub enum Value {
    Space(SpaceValue),
    Comma(CommaValue),
    Binary(BinaryValue),
    Fn(FnValue),
    Paren(ParenValue),
    Array(ArrayValue),
    Hash(HashValue),
    Unit(UnitValue),
    Percent(PercentValue),
    Brace(BraceValue),
    Url(UrlValue),
    Text(TextValue),
    String(StringValue),
    Number(NumberValue),
}

#[derive(Debug)]
pub struct SpaceValue {
    pub values: Vec<Value>,
}

#[derive(Debug)]
pub struct CommaValue {
    pub values: Vec<Value>,
}

#[derive(Debug)]
pub struct BinaryValue {
    pub left: Box<Value>,
    pub op: BinaryOperator,
    pub right: Box<Value>,
}

#[derive(Debug)]
pub enum BinaryOperator {
    /// `+`
    Add,
    /// `-`
    Sub,
    /// `*`
    Mul,
    /// `/`
    Div,
}

#[derive(Debug)]
pub struct FnValue {
    pub name: String,
    pub args: Vec<Value>,
}

#[derive(Debug)]
pub struct ParenValue {
    pub value: Option<Box<Value>>,
}

#[derive(Debug)]
pub struct ArrayValue {
    pub values: Vec<Value>,
}

#[derive(Debug)]
pub struct HashValue {
    pub value: String,
}

#[derive(Debug)]
pub struct UnitValue {
    pub value: String,
    pub unit: String,
}

#[derive(Debug)]
pub struct PercentValue {
    pub value: String,
}

#[derive(Debug)]
pub struct BraceValue {
    pub value: Box<Value>,
}

#[derive(Debug)]
pub struct UrlValue {
    pub url: String,
}

#[derive(Debug)]
pub struct TextValue {
    pub value: String,
}

#[derive(Debug)]
pub struct StringValue {
    pub value: String,
}

#[derive(Debug)]
pub struct NumberValue {
    pub value: String,
}
