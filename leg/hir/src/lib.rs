use la_arena::Idx;
use smol_str::SmolStr;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    VariableDef { name: SmolStr, value: Expr },
    Expr(Expr),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Missing,
    Binary {
        op: BinaryOp,
        lhs: Idx<Self>,
        rhs: Idx<Self>,
    },
    Unary {
        op: UnaryOp,
        expr: Idx<Self>,
    },
    VariableRef {
        var: SmolStr,
    },
    Literal(Literal),
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    /// The `+` operator (addition)
    Add,
    /// The `-` operator (subtraction)
    Sub,
    /// The `*` operator (multiplication)
    Mul,
    /// The `/` operator (division)
    Div,
    /// The `%` operator (modulus)
    Rem,
    /// The `&&` operator (logical and)
    And,
    /// The `||` operator (logical or)
    Or,
    /// The `==` operator (equality)
    Eq,
    // The `<` operator (less than)
    Lt,
    // The `<=` operator (less than or equal to)
    Le,
    // The `!=` operator (not equal to)
    Ne,
    // The `>` operator (greater than)
    Gt,
    // The `>=` operator (greater than or equal to)
    Ge,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    /// The `-` operator (negation)
    Neg,
    // The `<` operator (logical inversion)
    Not,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer { value: Option<u64> },
    String { value: SmolStr },
    Bool { value: bool },
}
