use super::{Expr, Token};

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(StmtExpr),
    Print(StmtPrint),
    Var(StmtVar),
    Block(StmtBlock),
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
