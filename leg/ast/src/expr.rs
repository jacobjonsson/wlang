use syntax::{syntax_kind::SyntaxKind, SyntaxElement, SyntaxNode, SyntaxToken};

use crate::token;

#[derive(Debug)]
pub enum Expr {
    BinaryExpr(BinaryExpr),
    Literal(Literal),
    ParenExpr(ParenExpr),
    UnaryExpr(UnaryExpr),
    VariableRef(VariableRef),
}

impl Expr {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        let result = match node.kind() {
            SyntaxKind::InfixExpr => Self::BinaryExpr(BinaryExpr(node)),
            SyntaxKind::Literal => Self::Literal(Literal(node)),
            SyntaxKind::ParenExpr => Self::ParenExpr(ParenExpr(node)),
            SyntaxKind::PrefixExpr => Self::UnaryExpr(UnaryExpr(node)),
            SyntaxKind::VariableRef => Self::VariableRef(VariableRef(node)),
            _ => return None,
        };

        Some(result)
    }
}

#[derive(Debug)]
pub struct BinaryExpr(SyntaxNode);

impl BinaryExpr {
    pub fn lhs(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }

    pub fn rhs(&self) -> Option<Expr> {
        self.0.children().filter_map(Expr::cast).nth(1)
    }

    pub fn op(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| {
                matches!(
                    token.kind(),
                    SyntaxKind::Plus
                        | SyntaxKind::Minus
                        | SyntaxKind::Star
                        | SyntaxKind::Slash
                        | SyntaxKind::Percent
                        | SyntaxKind::AmpersandAmpersand
                        | SyntaxKind::BarBar
                        | SyntaxKind::BangEquals
                        | SyntaxKind::EqualsEquals
                        | SyntaxKind::LessThan
                        | SyntaxKind::LessThanEqual
                        | SyntaxKind::GreaterThan
                        | SyntaxKind::GreaterThanEqual
                )
            })
    }
}

#[derive(Debug)]
pub enum LiteralKind {
    Integer(token::Integer),
    String(token::String),
    Bool(bool),
}

#[derive(Debug)]
pub struct Literal(SyntaxNode);

impl Literal {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == SyntaxKind::Literal {
            Some(Self(node))
        } else {
            None
        }
    }

    pub fn kind(&self) -> LiteralKind {
        let token = self.first_token().unwrap();

        if let Some(t) = token::Integer::cast(token.clone()) {
            return LiteralKind::Integer(t);
        }

        if let Some(t) = token::String::cast(token.clone()) {
            return LiteralKind::String(t);
        }

        match token.kind() {
            SyntaxKind::True => LiteralKind::Bool(true),
            SyntaxKind::False => LiteralKind::Bool(false),
            _ => unreachable!(),
        }
    }

    pub fn first_token(&self) -> Option<SyntaxToken> {
        self.0.first_token()
    }
}

#[derive(Debug)]
pub struct ParenExpr(SyntaxNode);

impl ParenExpr {
    pub fn expr(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }
}

#[derive(Debug)]
pub struct UnaryExpr(SyntaxNode);

impl UnaryExpr {
    pub fn expr(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }

    pub fn op(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| token.kind() == SyntaxKind::Minus)
    }
}

#[derive(Debug)]
pub struct VariableRef(SyntaxNode);

impl VariableRef {
    pub fn name(&self) -> Option<SyntaxToken> {
        self.0.first_token()
    }
}
