use std::cell::RefCell;
use std::fmt::Display;
use std::{collections::HashMap, rc::Rc};

use crate::lox::entities::expr::ExprKind;
use crate::lox::entities::Expr;
use crate::lox::entities::{eval::Interpreter, Token};

use super::visitor::StmtVisitor;
use super::{
    super::entities::{
        stmt::{StmtBlock, StmtExpr, StmtFun, StmtIf, StmtPrint, StmtReturn, StmtVar, StmtWhile},
        Stmt, Value,
    },
    visitor::ExprVisitor,
};
use log::{debug, trace};
use loxrs_env::Scope;
use loxrs_types::{LoxErr, Result};

#[derive(Debug)]
pub struct Resolver {
    interpreter: Rc<RefCell<Interpreter>>,
    stack: Vec<HashMap<String, bool>>,
}

impl Display for Resolver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Resolver: <")?;
        for el in &self.stack {
            write!(f, "[")?;
            for (k, v) in el.iter() {
                write!(f, "({}: {}) ", k, v)?;
            }
            write!(f, "]")?;
        }

        write!(f, ">")
    }
}

impl Resolver {
    pub fn new(interpreter: Rc<RefCell<Interpreter>>) -> Self {
        Self {
            interpreter,
            stack: vec![],
        }
    }

    fn resolve_fun_stmt(&mut self, stmt: &StmtFun) -> Result<Option<Value>> {
        self.begin_scope();

        for param in &stmt.def.params {
            self.declare(param)?;
            self.define(param)?;
        }
        self.resolve_stmt(&Stmt::Block(stmt.def.body.to_owned()))?;

        self.end_scope();
        Ok(None)
    }

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

    pub fn resolve(&mut self, stmts: &Vec<Stmt>) -> Result<Option<Value>> {
        for stmt in stmts {
            self.resolve_stmt(stmt)?;
        }

        trace!("resolver result: {}", self);
        Ok(None)
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<Option<Value>> {
        self.exec_stmt(stmt)
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<Option<Value>> {
        self.eval(expr)?;
        Ok(None)
    }

    fn resolve_local(&self, expr: &Expr, name: &str) -> Result<Option<Value>> {
        trace!("resolving to locals: {} with stack: {}", expr, self,);
        for (idx, scope) in self.stack.iter().rev().enumerate() {
            if scope.contains_key(name) {
                self.interpreter.as_ref().borrow_mut().resolve(expr, idx);
                return Ok(None);
            }
        }

        Ok(None)
    }
}

impl Interpreter {
    pub fn resolve(&self, expr: &Expr, idx: usize) {
        self.locals.borrow_mut().insert(expr.to_owned(), idx);
    }
}

impl StmtVisitor for Resolver {
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
        debug!("resolver result for {}: {:?}", stmt, res);
        res
    }

    fn print_stmt(&mut self, stmt: &StmtPrint) -> Result<Option<Value>> {
        self.resolve_expr(&stmt.expr)
    }

    fn eval_stmt(&mut self, stmt: &StmtExpr) -> Result<Option<Value>> {
        self.resolve_expr(&stmt.expr)
    }

    fn return_stmt(&mut self, stmt: &StmtReturn) -> Result<Option<Value>> {
        self.resolve_expr(&stmt.val)
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
        self.declare(&stmt.name)?;
        self.define(&stmt.name)?;
        self.resolve_fun_stmt(stmt)
    }

    fn block_stmt(
        &mut self,
        block: &StmtBlock,
        _scope: Rc<loxrs_env::Scope<Value>>,
    ) -> Result<Option<Value>> {
        self.begin_scope();
        self.resolve(&block.stmts)?;
        self.end_scope();

        Ok(None)
    }

    fn if_stmt(&mut self, stmt: &StmtIf) -> Result<Option<Value>> {
        self.resolve_expr(&stmt.cond)?;
        self.resolve_stmt(&stmt.then)?;
        if let Some(else_stmt) = &stmt.else_stmt {
            self.resolve_stmt(else_stmt.as_ref())?;
        }

        Ok(None)
    }

    fn while_stmt(&mut self, stmt: &StmtWhile) -> Result<Option<Value>> {
        self.resolve_expr(&stmt.expr)?;
        self.resolve_stmt(&stmt.stmt)
    }
}

impl ExprVisitor<Option<Value>> for Resolver {
    fn func(&mut self, def: &crate::lox::entities::expr::ExprFunction) -> Result<Option<Value>> {
        self.resolve_stmt(&Stmt::Block(StmtBlock {
            stmts: def.body.stmts.to_owned(),
        }))
    }

    fn literal(&mut self, _literal: &crate::lox::entities::Literal) -> Result<Option<Value>> {
        Ok(None)
    }

    fn unary(&mut self, right: &Expr, _operator: &Token) -> Result<Option<Value>> {
        self.resolve_expr(right)
    }

    fn binary(&mut self, left: &Expr, right: &Expr, _operator: &Token) -> Result<Option<Value>> {
        self.resolve_expr(left)?;
        self.resolve_expr(right)
    }

    fn grouping(
        &mut self,
        expression: &crate::lox::entities::expr::ExprGrouping,
    ) -> Result<Option<Value>> {
        self.resolve_expr(&expression.expression)
    }

    fn var(&self, expression: &Expr) -> Result<Option<Value>> {
        if let ExprKind::Var(var) = &expression.kind {
            let name = var.extract_identifier_str()?;
            if self
                .stack
                .last()
                .is_some_and(|last| last.get(name).is_some_and(|el| !*el))
            {
                return Err(LoxErr::Resolve {
                    message: format!("Can't read local variable {} from its own initalizer", name),
                });
            }

            self.resolve_local(expression, name)?;
            return Ok(None);
        }

        Err(LoxErr::Internal {
            message: format!(
                "{} not expected in `var` code path, programmer error",
                expression
            ),
        })
    }

    fn assign(
        &mut self,
        token: &Token,
        expr: &crate::lox::entities::expr::ExprAssign,
    ) -> Result<Option<Value>> {
        self.resolve_expr(&expr.expression)?;
        self.resolve_local(&expr.expression, token.extract_identifier_str()?)
    }

    fn logical(&mut self, left: &Expr, right: &Expr, _operator: &Token) -> Result<Option<Value>> {
        self.resolve_expr(left)?;
        self.resolve_expr(right)
    }

    fn call(&mut self, callee: &Expr, args: &[Expr]) -> Result<Option<Value>> {
        self.resolve_expr(callee)?;
        for expr in args {
            self.resolve_expr(expr)?;
        }
        Ok(None)
    }
}