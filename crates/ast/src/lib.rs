#![allow(dead_code)]

/// A file node represents a W source file.
pub struct File {
    decls: Vec<DeclKind>,
}

pub enum DeclKind {
    /// A function declaration
    Fn,
    /// A constant declaration
    Const,
}

pub enum StmtKind {
    /// A let statement
    Let,
}

pub enum ExprKind {}
