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
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    Neg,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer { value: Option<u64> },
}
