use std::{collections::HashMap, rc::Rc};

use crate::lox::entities::{eval::Interpreter, Token};

use super::visitor::StmtVisitor;
use super::{
    super::entities::{
        stmt::{StmtBlock, StmtExpr, StmtFun, StmtIf, StmtPrint, StmtReturn, StmtVar, StmtWhile},
        Expr, Stmt, Value,
    },
    visitor::ExprVisitor,
};
use log::debug;
use loxrs_env::Scope;
use loxrs_types::{LoxErr, Result};

#[derive(Debug)]
pub struct Resolver {
    interpreter: Rc<Interpreter>,
    stack: Vec<HashMap<String, bool>>,
}

impl Resolver {
    fn declare(&mut self, name: &Token) -> Result<Option<Value>> {
        if let Some(last) = self.stack.last_mut() {
            last.insert(name.extract_identifier_str()?.to_owned(), false);
        }

        Ok(None)
    }

    fn define(&mut self, name: &Token) -> Result<Option<Value>> {
        if let Some(last) = self.stack.last_mut() {
            last.insert(name.extract_identifier_str()?.to_owned(), true);
        }

        Ok(None)
    }

    fn begin_scope(&mut self) {
        self.stack.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.stack.pop();
    }

    fn resolve(&mut self, stmts: &Vec<Stmt>) -> Result<Option<Value>> {
        for stmt in stmts {
            self.resolve_stmt(stmt)?;
        }

        Ok(None)
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<Option<Value>> {
        self.exec_stmt(stmt)
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<Option<Value>> {
        self.eval(expr)?;
        Ok(None)
    }

    fn resolve_local(&self) -> _ {
        // TODO left off here
        todo!()
    }
}

impl StmtVisitor for Resolver {
    // TODO refactor so that this is a default implementation
    fn exec_stmt(&mut self, stmt: &Stmt) -> Result<Option<Value>> {
        let res = match stmt {
            Stmt::Print(stmt) => self.print_stmt(stmt),
            Stmt::Return(stmt) => self.return_stmt(stmt),
            Stmt::Expr(stmt) => self.eval_stmt(stmt),
            Stmt::Fun(stmt) => self.fun_stmt(stmt),
            Stmt::Var(stmt) => self.var_stmt(stmt),
            Stmt::Block(stmt) => self.block_stmt(stmt, Rc::new(Scope::new())),
            Stmt::If(stmt) => self.if_stmt(stmt),
            Stmt::While(stmt) => self.while_stmt(stmt),
        };
        debug!("statement execution result for {}: {:?}", stmt, res);
        res
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
        self.declare(&var.token)?;
        if let Some(expr) = &var.expr {
            self.resolve_expr(expr)?;
        }

        self.define(&var.token)?;
        Ok(None)
    }

    fn fun_stmt(&mut self, stmt: &StmtFun) -> Result<Option<Value>> {
        todo!()
    }

    fn block_stmt(
        &mut self,
        block: &StmtBlock,
        _scope: Rc<loxrs_env::Scope<Value>>,
    ) -> Result<Option<Value>> {
        self.begin_scope();
        self.resolve(&block.stmts);
        self.end_scope();

        Ok(None)
    }

    fn if_stmt(&mut self, stmt: &StmtIf) -> Result<Option<Value>> {
        todo!()
    }

    fn while_stmt(&mut self, stmt: &StmtWhile) -> Result<Option<Value>> {
        todo!()
    }
}

impl ExprVisitor<Option<Value>> for Resolver {
    fn func(&mut self, def: &crate::lox::entities::expr::ExprFunction) -> Result<Option<Value>> {
        todo!()
    }

    fn literal(&mut self, literal: &crate::lox::entities::Literal) -> Result<Option<Value>> {
        todo!()
    }

    fn unary(&mut self, right: &Expr, operator: &Token) -> Result<Option<Value>> {
        todo!()
    }

    fn binary(&mut self, left: &Expr, right: &Expr, operator: &Token) -> Result<Option<Value>> {
        todo!()
    }

    fn grouping(
        &mut self,
        expression: &crate::lox::entities::expr::ExprGrouping,
    ) -> Result<Option<Value>> {
        todo!()
    }

    fn var(&self, expression: &Token) -> Result<Option<Value>> {
        let var_str = expression.extract_identifier_str()?.to_owned();
        if self
            .stack
            .last()
            .is_some_and(|last| last.get(&var_str).is_some_and(|el| *el == false))
        {
            return Err(LoxErr::Resolve {
                message: format!(
                    "Can't read local variable {} from its own initalizer",
                    var_str
                ),
            });
        }

        self.resolve_local();
        Ok(None)
    }

    fn assign(
        &mut self,
        token: &Token,
        expr: &crate::lox::entities::expr::ExprAssign,
    ) -> Result<Option<Value>> {
        todo!()
    }

    fn logical(&mut self, left: &Expr, right: &Expr, operator: &Token) -> Result<Option<Value>> {
        todo!()
    }

    fn call(&mut self, callee: &Expr, args: &[Expr]) -> Result<Option<Value>> {
        todo!()
    }
}
