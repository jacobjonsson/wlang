use la_arena::Arena;
use syntax::SyntaxKind;

/// Lowers the given AST into HIR
pub fn lower_root(root: ast::Root) -> (Database, Vec<hir::Stmt>) {
    let mut database = Database::default();
    let stmts = root
        .stmts()
        .filter_map(|stmt| database.lower_stmt(stmt))
        .collect();

    (database, stmts)
}

#[derive(Default, Debug, PartialEq)]
pub struct Database {
    exprs: Arena<hir::Expr>,
}

impl Database {
    pub fn lower_stmt(&mut self, stmt: ast::Stmt) -> Option<hir::Stmt> {
        let result = match stmt {
            ast::Stmt::VariableDef(variable_def) => hir::Stmt::VariableDef {
                name: variable_def.name()?.text().into(),
                value: self.lower_expr(variable_def.value()),
            },
            ast::Stmt::Expr(expr) => hir::Stmt::Expr(self.lower_expr(Some(expr))),
        };

        Some(result)
    }

    fn lower_expr(&mut self, expr: Option<ast::Expr>) -> hir::Expr {
        if let Some(expr) = expr {
            match expr {
                ast::Expr::BinaryExpr(expr) => self.lower_binary_expr(expr),
                ast::Expr::Literal(expr) => self.lower_literal(expr),
                ast::Expr::ParenExpr(expr) => self.lower_paren_expr(expr),
                ast::Expr::UnaryExpr(expr) => self.lower_unary_expr(expr),
                ast::Expr::VariableRef(expr) => self.lower_variable_ref(expr),
            }
        } else {
            hir::Expr::Missing
        }
    }

    fn lower_binary_expr(&mut self, expr: ast::BinaryExpr) -> hir::Expr {
        let op = match expr.op().unwrap().kind() {
            SyntaxKind::Plus => hir::BinaryOp::Add,
            SyntaxKind::Minus => hir::BinaryOp::Sub,
            SyntaxKind::Slash => hir::BinaryOp::Div,
            SyntaxKind::Star => hir::BinaryOp::Mul,
            _ => unreachable!(),
        };

        let lhs = self.lower_expr(expr.lhs());
        let rhs = self.lower_expr(expr.rhs());

        hir::Expr::Binary {
            lhs: self.exprs.alloc(lhs),
            op,
            rhs: self.exprs.alloc(rhs),
        }
    }

    fn lower_unary_expr(&mut self, expr: ast::UnaryExpr) -> hir::Expr {
        let op = match expr.op().unwrap().kind() {
            SyntaxKind::Minus => hir::UnaryOp::Neg,
            _ => unreachable!(),
        };

        let inner_expr = self.lower_expr(expr.expr());

        hir::Expr::Unary {
            expr: self.exprs.alloc(inner_expr),
            op,
        }
    }

    fn lower_literal(&mut self, expr: ast::Literal) -> hir::Expr {
        let value = expr.parse();
        hir::Expr::Literal { n: value }
    }

    fn lower_paren_expr(&mut self, expr: ast::ParenExpr) -> hir::Expr {
        self.lower_expr(expr.expr())
    }

    fn lower_variable_ref(&mut self, expr: ast::VariableRef) -> hir::Expr {
        let name = expr.name().unwrap().text().into();
        hir::Expr::VariableRef { var: name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Database;
    use hir::*;

    fn parse(input: &str) -> ast::Root {
        ast::Root::cast(parser::parse(input).syntax()).unwrap()
    }

    fn check_stmt(input: &str, expected_hir: Stmt) {
        let root = parse(input);
        let ast = root.stmts().next().unwrap();
        let hir = Database::default().lower_stmt(ast).unwrap();

        assert_eq!(hir, expected_hir);
    }

    fn check_expr(input: &str, expected_hir: Expr, expected_database: Database) {
        let root = parse(input);
        let first_stmt = root.stmts().next().unwrap();
        let ast = match first_stmt {
            ast::Stmt::Expr(ast) => ast,
            _ => unreachable!(),
        };
        let mut database = Database::default();
        let hir = database.lower_expr(Some(ast));

        assert_eq!(hir, expected_hir);
        assert_eq!(database, expected_database);
    }

    #[test]
    fn lower_variable_def() {
        check_stmt(
            "let foo = bar",
            Stmt::VariableDef {
                name: "foo".into(),
                value: Expr::VariableRef { var: "bar".into() },
            },
        )
    }

    #[test]
    fn lower_variable_def_without_name() {
        let root = parse("let = 10");
        let ast = root.stmts().next().unwrap();
        assert!(Database::default().lower_stmt(ast).is_none());
    }

    #[test]
    fn lower_variable_def_without_value() {
        check_stmt(
            "let a =",
            Stmt::VariableDef {
                name: "a".into(),
                value: Expr::Missing,
            },
        )
    }

    #[test]
    fn lower_expr_stmt() {
        check_stmt("123", Stmt::Expr(Expr::Literal { n: Some(123) }))
    }

    #[test]
    fn lower_binary_expr() {
        let mut exprs = Arena::new();
        let lhs = exprs.alloc(Expr::Literal { n: Some(1) });
        let rhs = exprs.alloc(Expr::Literal { n: Some(2) });

        check_expr(
            "1 + 2",
            Expr::Binary {
                lhs,
                op: BinaryOp::Add,
                rhs,
            },
            Database { exprs },
        );
    }

    #[test]
    fn lower_binary_expr_without_rhs() {
        let mut exprs = Arena::new();
        let lhs = exprs.alloc(Expr::Literal { n: Some(1) });
        let rhs = exprs.alloc(Expr::Missing);

        check_expr(
            "1 + ",
            Expr::Binary {
                lhs,
                op: BinaryOp::Add,
                rhs,
            },
            Database { exprs },
        );
    }

    #[test]
    fn lower_literal() {
        check_expr("1", Expr::Literal { n: Some(1) }, Database::default())
    }

    #[test]
    fn lower_paren_expr() {
        check_expr(
            "(((((abc)))))",
            Expr::VariableRef { var: "abc".into() },
            Database::default(),
        )
    }

    #[test]
    fn test_unary_expr() {
        let mut exprs = Arena::new();
        let expr = exprs.alloc(Expr::Literal { n: Some(10) });

        check_expr(
            "-10",
            Expr::Unary {
                op: UnaryOp::Neg,
                expr,
            },
            Database { exprs },
        )
    }

    #[test]
    fn lower_unary_expr_without_expr() {
        let mut exprs = Arena::new();
        let expr = exprs.alloc(Expr::Missing);

        check_expr(
            "-",
            Expr::Unary {
                op: UnaryOp::Neg,
                expr,
            },
            Database { exprs },
        );
    }

    #[test]
    fn test_variable_ref() {
        check_expr(
            "abc",
            Expr::VariableRef { var: "abc".into() },
            Database::default(),
        )
    }
}
