use std::cell::RefCell;
use std::fmt::Debug;

use super::Value;
use loxrs_env::Env;

#[derive(Debug, Clone)]
pub struct Interpreter {
    pub env: RefCell<Env<Value>>,
}

// pub trait StmtExec<T: Debug + Clone = Result<Option<Value>>> {
//     fn exec_stmt(&mut self, stmt: &Stmt) -> T;
//     fn print_stmt(&mut self, expr: &StmtPrint) -> T;
//     fn eval_stmt(&mut self, expr: &StmtExpr) -> T;
//     fn return_stmt(&mut self, expr: &StmtReturn) -> T;
//     fn fun_stmt(&mut self, expr: &StmtFun) -> T;
//     fn var_stmt(&mut self, token: &StmtVar) -> T;
//     fn block_stmt(&mut self, expr: &StmtBlock, env: RefCell<Env<Value>>) -> T;
//     fn if_stmt(&mut self, stmt: &StmtIf) -> T;
//     fn while_stmt(&mut self, stmt: &StmtWhile) -> T;
// }
//
// pub trait ExprEval<T: Debug + Clone = Result<Value>> {
//     fn eval(&mut self, expr: &Expr) -> T {
//         match expr {
//             Expr::Unary(unary) => self.unary(&unary.right, &unary.operator),
//             Expr::Binary(binary) => self.binary(&binary.left, &binary.right, &binary.operator),
//             Expr::Logical(logical) => {
//                 self.logical(&logical.left, &logical.right, &logical.operator)
//             }
//             Expr::Grouping(grouping) => self.grouping(grouping),
//             Expr::Literal(lit) => self.literal(lit),
//             Expr::Var(var) => self.var(var),
//             Expr::Assign(token, expr) => self.assign(token, expr),
//             Expr::Call(call) => self.call(&call.callee, &call.args),
//         }
//     }
//     fn literal(&mut self, literal: &Literal) -> T;
//     fn unary(&mut self, right: &Expr, operator: &Token) -> T;
//     fn binary(&mut self, left: &Expr, right: &Expr, operator: &Token) -> T;
//     fn grouping(&mut self, expression: &ExprGrouping) -> T;
//     fn var(&self, expression: &Token) -> T;
//     fn assign(&mut self, token: &Token, expr: &ExprAssign) -> T;
//     fn logical(&mut self, left: &Expr, right: &Expr, operator: &Token) -> T;
//     fn call(&mut self, callee: &Expr, args: &[Expr]) -> T;
// }
