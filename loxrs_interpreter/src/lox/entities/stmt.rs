use std::fmt::Display;

use super::{expr::ExprFunction, Expr, Token};

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Fun(StmtFun),
    Return(StmtReturn),
    Expr(StmtExpr),
    Print(StmtPrint),
    Var(StmtVar),
    Block(StmtBlock),
    If(StmtIf),
    While(StmtWhile),
}

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Fun(_) => "[Stmt]Fun",
                Self::Return(_) => "[Stmt]Return",
                Self::Expr(_) => "[Stmt]Expr",
                Self::Print(_) => "[Stmt]Print",
                Self::Var(_) => "[Stmt]Var",
                Self::Block(_) => "[Stmt]Block",
                Self::If(_) => "[Stmt]If",
                Self::While(_) => "[Stmt]While",
            }
        )
    }
}

type StmtB = Box<Stmt>;

#[derive(Debug, Clone, PartialEq)]
pub struct StmtReturn {
    pub keyword: Token,
    pub val: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StmtFun {
    pub name: Token,
    pub def: ExprFunction,
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
    pub stmt: StmtB,
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StmtIf {
    pub cond: Expr,
    pub then: StmtB,
    pub else_stmt: Option<StmtB>,
}
