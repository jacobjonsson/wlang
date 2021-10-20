use crate::Value;

pub struct Property {
    pub name: String,
    pub values: Vec<Value>,
    pub important: bool,
}
