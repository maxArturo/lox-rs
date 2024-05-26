use crate::lox::entities::expr::ExprKind;

use super::super::entities::{
    expr::{ExprFunction, ExprGrouping},
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
        match &expr.kind {
            ExprKind::Unary(unary) => self.unary(&unary.right, &unary.operator),
            ExprKind::Binary(binary) => self.binary(&binary.left, &binary.right, &binary.operator),
            ExprKind::Logical(logical) => {
                self.logical(&logical.left, &logical.right, &logical.operator)
            }
            ExprKind::Grouping(grouping) => self.grouping(grouping.as_ref()),
            ExprKind::Function(func) => self.func(func.as_ref()),
            ExprKind::Literal(lit) => self.literal(lit.as_ref()),
            ExprKind::Var(_) => self.var(expr),
            ExprKind::Assign(_) => self.assign(expr),
            ExprKind::Call(call) => self.call(&call.callee, &call.args),
        }
    }

    fn func(&mut self, def: &ExprFunction) -> Result<T>;

    fn literal(&mut self, literal: &Literal) -> Result<T>;

    fn unary(&mut self, right: &Expr, operator: &Token) -> Result<T>;

    fn binary(&mut self, left: &Expr, right: &Expr, operator: &Token) -> Result<T>;

    fn grouping(&mut self, expression: &ExprGrouping) -> Result<T>;

    fn var(&self, expression: &Expr) -> Result<T>;

    fn assign(&mut self, expr: &Expr) -> Result<T>;

    fn logical(&mut self, left: &Expr, right: &Expr, operator: &Token) -> Result<T>;

    fn call(&mut self, callee: &Expr, args: &[Expr]) -> Result<T>;
}
