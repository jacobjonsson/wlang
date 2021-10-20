pub enum Unit {
    Px,
    Rem,
    Em,
}

pub enum Value {
    Paren {
        values: Option<Vec<Value>>,
    },
    Unit {
        value: f64,
        unit: Unit,
    },
    Number {
        value: f64,
    },
    Percent {
        value: f64,
    },
    Hash {
        value: String,
    },
    Text {
        value: String,
    },
    String {
        value: String,
    },
    Fn {
        name: String,
        args: Vec<Value>,
    },
    Bin {
        left: Box<Value>,
        op: BinOp,
        right: Box<Value>,
    },
    Array {
        values: Vec<Value>,
    },
    Space {
        values: Vec<Value>,
    },
    Comma {
        values: Vec<Value>,
    },
    Brace {
        value: Box<Value>,
    },
    Url {
        url: String,
    },
}

pub enum BinOp {
    /// `+`
    Add,
    /// `-`
    Sub,
    /// `*`
    Mul,
    /// `/`
    Div,
}
