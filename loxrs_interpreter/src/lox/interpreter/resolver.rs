use std::rc::Rc;

use crate::lox::entities::eval::Interpreter;

use super::super::entities::{
    expr::{ExprAssign, ExprGrouping},
    stmt::{StmtBlock, StmtExpr, StmtFun, StmtIf, StmtPrint, StmtReturn, StmtVar, StmtWhile},
    Expr, Literal, Stmt, Token, TokenType, Value,
};
use super::visitor::StmtVisitor;
use loxrs_types::{LoxErr, Result};

#[derive(Debug)]
pub struct Resolver {
    interpreter: Rc<Interpreter>,
}

impl Resolver {
    fn beginScope() {
        todo!();
    }
    fn endScope() {
        todo!();
    }
}

impl StmtVisitor for Resolver {
    fn exec_stmt(&mut self, stmt: &Stmt) -> Result<Option<Value>> {
        todo!()
    }

    fn print_stmt(&mut self, stmt: &StmtPrint) -> Result<Option<Value>> {
        todo!()
    }

    fn eval_stmt(&mut self, stmt: &StmtExpr) -> Result<Option<Value>> {
        todo!()
    }

    fn return_stmt(&mut self, stmt: &StmtReturn) -> Result<Option<Value>> {
        todo!()
    }

    fn var_stmt(&mut self, var: &StmtVar) -> Result<Option<Value>> {
        todo!()
    }

    fn fun_stmt(&mut self, stmt: &StmtFun) -> Result<Option<Value>> {
        todo!()
    }

    fn block_stmt(
        &mut self,
        block: &StmtBlock,
        scope: Rc<loxrs_env::Scope<Value>>,
    ) -> Result<Option<Value>> {
        self.beginScope();
        self.endScope();
    }

    fn if_stmt(&mut self, stmt: &StmtIf) -> Result<Option<Value>> {
        todo!()
    }

    fn while_stmt(&mut self, stmt: &StmtWhile) -> Result<Option<Value>> {
        todo!()
    }
}
