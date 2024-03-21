use super::{Expr, Token};

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(StmtExpr),
    Print(StmtPrint),
    Var(StmtVar),
    Block(StmtBlock),
    If(StmtIf),
    While(StmtWhile),
}

#[derive(Debug, Clone, PartialEq)]
pub struct StmtExpr {
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StmtPrint {
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StmtVar {
    pub token: Token,
    pub expr: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StmtBlock {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StmtWhile {
    pub stmt: Box<Stmt>,
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StmtIf {
    pub cond: Expr,
    pub then: Box<Stmt>,
    pub else_stmt: Option<Box<Stmt>>,
}
