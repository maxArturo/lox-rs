use super::super::entities::{
    expr::{ExprAssign, ExprFunction, ExprGrouping},
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

pub trait ExprVisitor<T> {
    fn eval(&mut self, expr: &Expr) -> Result<T> {
        match expr {
            Expr::Unary(unary) => self.unary(&unary.right, &unary.operator),
            Expr::Binary(binary) => self.binary(&binary.left, &binary.right, &binary.operator),
            Expr::Logical(logical) => {
                self.logical(&logical.left, &logical.right, &logical.operator)
            }
            Expr::Grouping(grouping) => self.grouping(grouping),
            Expr::Function(func) => self.func(func),
            Expr::Literal(lit) => self.literal(lit),
            Expr::Var(var) => self.var(var),
            Expr::Assign(token, expr) => self.assign(token, expr),
            Expr::Call(call) => self.call(&call.callee, &call.args),
        }
    }

    fn func(&mut self, def: &ExprFunction) -> Result<T>;

    fn literal(&mut self, literal: &Literal) -> Result<T>;

    fn unary(&mut self, right: &Expr, operator: &Token) -> Result<T>;

    fn binary(&mut self, left: &Expr, right: &Expr, operator: &Token) -> Result<T>;

    fn grouping(&mut self, expression: &ExprGrouping) -> Result<T>;

    fn var(&self, expression: &Token) -> Result<T>;

    fn assign(&mut self, token: &Token, expr: &ExprAssign) -> Result<T>;

    fn logical(&mut self, left: &Expr, right: &Expr, operator: &Token) -> Result<T>;

    fn call(&mut self, callee: &Expr, args: &[Expr]) -> Result<T>;
}
