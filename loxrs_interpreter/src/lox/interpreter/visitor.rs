use super::super::entities::{
    expr::{ExprAssign, ExprGrouping, ExprFunction},
    stmt::{StmtBlock, StmtExpr, StmtFun, StmtIf, StmtPrint, StmtReturn, StmtVar, StmtWhile},
    Expr, Literal, Stmt, Token, Value,
};

use loxrs_env::Scope;
use loxrs_types::Result;
use std::rc::Rc;

pub trait StmtVisitor {
    fn exec_stmt(&mut self, stmt: &Stmt) -> Result<Option<Value>>;

    fn print_stmt(&mut self, stmt: &StmtPrint) -> Result<Option<Value>>;

    fn eval_stmt(&mut self, stmt: &StmtExpr) -> Result<Option<Value>>;

    fn return_stmt(&mut self, stmt: &StmtReturn) -> Result<Option<Value>>;

    fn var_stmt(&mut self, var: &StmtVar) -> Result<Option<Value>>;

    fn fun_stmt(&mut self, stmt: &StmtFun) -> Result<Option<Value>>;

    fn block_stmt(&mut self, block: &StmtBlock, scope: Rc<Scope<Value>>) -> Result<Option<Value>>;

    fn if_stmt(&mut self, stmt: &StmtIf) -> Result<Option<Value>>;

    fn while_stmt(&mut self, stmt: &StmtWhile) -> Result<Option<Value>>;
}

pub trait ExprVisitor {
    fn eval(&mut self, expr: &Expr) -> Result<Value>;

    fn func(&mut self, def: &ExprFunction) -> Result<Value>;

    fn literal(&mut self, literal: &Literal) -> Result<Value>;

    fn unary(&mut self, right: &Expr, operator: &Token) -> Result<Value>;

    fn binary(&mut self, left: &Expr, right: &Expr, operator: &Token) -> Result<Value>;

    fn grouping(&mut self, expression: &ExprGrouping) -> Result<Value>;

    fn var(&self, expression: &Token) -> Result<Value>;

    fn assign(&mut self, token: &Token, expr: &ExprAssign) -> Result<Value>;

    fn logical(&mut self, left: &Expr, right: &Expr, operator: &Token) -> Result<Value>;

    fn call(&mut self, callee: &Expr, args: &[Expr]) -> Result<Value>;
}
