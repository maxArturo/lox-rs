use std::fmt::{Display, Error, Formatter};

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
//
// impl Display for Stmt {
//     fn fmt(&self, f: &mut std::Formatter<'_>) -> std::Result {
//         write!(
//             f,
//             "{}",
//             match self {
//                 Self::Fun(_) => "[Stmt]Fun",
//                 Self::Return(_) => "[Stmt]Return",
//                 Self::Expr(_) => "[Stmt]Expr",
//                 Self::Print(_) => "[Stmt]Print",
//                 Self::Var(_) => "[Stmt]Var",
//                 Self::Block(_) => "[Stmt]Block",
//                 Self::If(_) => "[Stmt]If",
//                 Self::While(_) => "[Stmt]While",
//             }
//         )
//     }
// }
//
impl Display for Stmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Fun(stmt) => write!(f, "[Stmt]Function: {}", stmt),
            Stmt::Return(stmt) => write!(f, "[Stmt]Return: {}", stmt),
            Stmt::Expr(stmt) => write!(f, "[Stmt]Expr: {}", stmt),
            Stmt::Print(stmt) => write!(f, "[Stmt]Print: {}", stmt),
            Stmt::Var(stmt) => write!(f, "[Stmt]Var: {}", stmt),
            Stmt::Block(stmt) => write!(f, "[Stmt]Block: {}", stmt),
            Stmt::If(stmt) => write!(f, "[Stmt]If: {}", stmt),
            Stmt::While(stmt) => write!(f, "[Stmt]While: {}", stmt),
        }
    }
}

impl Display for StmtReturn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.keyword, self.val)
    }
}

impl Display for StmtFun {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.name, self.def)
    }
}

impl Display for StmtExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expr)
    }
}

impl Display for StmtPrint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expr)
    }
}

impl Display for StmtVar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.expr {
            Some(expr) => write!(f, "{} = {}", self.token, expr),
            None => write!(f, "{}", self.token),
        }
    }
}

impl Display for StmtBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for stmt in &self.stmts {
            writeln!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

impl Display for StmtWhile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "while {} {}", self.expr, self.stmt)
    }
}

impl Display for StmtIf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "if {} {}", self.cond, self.then)?;
        if let Some(else_stmt) = &self.else_stmt {
            write!(f, " else {}", else_stmt)?;
        }
        Ok(())
    }
}
