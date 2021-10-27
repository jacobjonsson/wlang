use std::ops::Index;

use ast_lowering::Database;
use hir::{BinaryOp, Expr, Literal, Stmt, UnaryOp};

pub fn generate(statements: Vec<Stmt>, database: Database) -> String {
    let mut generator = Generator {
        source: String::new(),
    };

    for statement in statements {
        generator.generate_statement(&statement, &database);
    }

    generator.source
}

pub struct Generator {
    source: String,
}

impl Generator {
    fn generate_statement(&mut self, statement: &Stmt, database: &Database) {
        match statement {
            Stmt::VariableDef { name, value } => {
                self.source.push_str(&format!("let {} = ", name));
                self.generate_expression(value, database);
            }
            Stmt::Expr(expr) => self.generate_expression(expr, database),
        }
    }

    fn generate_expression(&mut self, expression: &Expr, database: &Database) {
        match &expression {
            Expr::Missing => self.source.push_str(""),
            Expr::Binary { op, lhs, rhs } => {
                let lhs = database.exprs().index(*lhs);
                let rhs = database.exprs().index(*rhs);

                self.generate_expression(lhs, database);
                self.source.push_str(" ");
                match op {
                    BinaryOp::Add => self.source.push_str("+"),
                    BinaryOp::Sub => self.source.push_str("-"),
                    BinaryOp::Mul => self.source.push_str("*"),
                    BinaryOp::Div => self.source.push_str("/"),
                    BinaryOp::Rem => self.source.push_str("%"),
                    BinaryOp::And => self.source.push_str("&&"),
                    BinaryOp::Or => self.source.push_str("||"),
                    BinaryOp::Eq => self.source.push_str("=="),
                    BinaryOp::Lt => self.source.push_str("<"),
                    BinaryOp::Le => self.source.push_str("<="),
                    BinaryOp::Ne => self.source.push_str("!="),
                    BinaryOp::Gt => self.source.push_str(">"),
                    BinaryOp::Ge => self.source.push_str(">="),
                }
                self.source.push_str(" ");
                self.generate_expression(rhs, database);
            }
            Expr::Unary { op, expr } => {
                match op {
                    UnaryOp::Neg => self.source.push_str("-"),
                    UnaryOp::Not => self.source.push_str("!"),
                }

                let expr = database.exprs().index(*expr);
                self.generate_expression(expr, database);
            }
            Expr::VariableRef { var } => self.source.push_str(var),
            Expr::Literal(kind) => match kind {
                Literal::Integer { value } => self.source.push_str(&value.unwrap().to_string()),
                Literal::String { value } => self.source.push_str(value),
                Literal::Bool { value } => self.source.push_str(&value.to_string()),
            },
        }
    }
}
