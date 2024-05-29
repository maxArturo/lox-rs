use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::{collections::HashMap, rc::Rc};

use crate::lox::entities::expr::{ExprFunction, ExprKind};
use crate::lox::entities::func::FuncType;
use crate::lox::entities::stmt::StmtClass;
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
use log::trace;
use loxrs_env::Scope;
use loxrs_types::{LoxErr, Result};

#[derive(Default, Debug, Clone, PartialEq)]
enum VarStatus {
    #[default]
    Declared,
    Defined,
    Assigned,
}

impl Display for VarStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Declared => "declared",
                Self::Defined => "defined",
                Self::Assigned => "assigned",
            }
        )
    }
}

#[derive(Debug)]
pub struct Resolver {
    interpreter: Rc<RefCell<Interpreter>>,
    // TODO here this needs to be changed into an enum
    // and on `assign` it needs to be set to assigned
    // when we pop a scope, any unassigned vars should be
    // an error
    stack: Vec<HashMap<String, VarStatus>>,
    curr_function: FuncType,
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
            curr_function: FuncType::None,
        }
    }

    fn resolve_fun_stmt(&mut self, stmt: &StmtFun) -> Result<Option<Value>> {
        trace!(
            "creating stack from fun stmt: {} with stack: {}",
            stmt,
            self,
        );

        let prev_function_type = self.curr_function.clone();
        self.curr_function = FuncType::Function;

        self.begin_scope();

        for param in &stmt.def.params {
            self.declare(param)?;
            self.define(param)?;
            self.assign(param)?;
        }
        self.resolve_stmt(&Stmt::Block(stmt.def.body.to_owned()))?;

        self.end_scope()?;

        self.curr_function = prev_function_type;
        Ok(None)
    }

    fn declare(&mut self, name: &Token) -> Result<Option<Value>> {
        if let Some(last) = self.stack.last_mut() {
            let name_val = name.extract_identifier_str()?;
            if last.get(name_val).is_some() {
                return Err(LoxErr::Resolve {
                    message: format!(
                        "Variable `{}` already declared in current scope\n in line: {}, col: {}",
                        name_val, name.line, name.column
                    ),
                });
            }

            last.insert(
                name.extract_identifier_str()?.to_owned(),
                VarStatus::default(),
            );
        }

        Ok(None)
    }

    fn define(&mut self, name: &Token) -> Result<Option<Value>> {
        let var_name = name.extract_identifier_str()?;
        if let Some(last) = self.stack.last_mut() {
            if !last.contains_key(var_name) {
                return Err(LoxErr::Resolve {
                            message: format!("Can't define local variable {} before it is declared\n at line: {}, col: {}", 
                                var_name, name.line,
                                name.column),
                        });
            }
            last.insert(
                name.extract_identifier_str()?.to_owned(),
                VarStatus::Defined,
            );
        }

        Ok(None)
    }

    fn assign(&mut self, name: &Token) -> Result<Option<Value>> {
        let var_name = name.extract_identifier_str()?;
        if let Some(last) = self.stack.last_mut() {
            if !last
                .get(var_name)
                .is_some_and(|el| *el == VarStatus::Defined)
            {
                return Err(LoxErr::Resolve {
                    message: format!("Can't assign local variable {} before it is defined\n at line: {}, col: {}", var_name, name.line,
                        name.column),
                });
            }
            last.insert(var_name.to_owned(), VarStatus::Assigned);
        }

        Ok(None)
    }

    fn begin_scope(&mut self) {
        self.stack.push(HashMap::new());
    }

    fn end_scope(&mut self) -> Result<Option<Value>> {
        if let Some(stack) = self.stack.pop() {
            for (k, v) in stack {
                if v != VarStatus::Assigned {
                    return Err(LoxErr::Resolve {
                        message: format!("Variable `{}` not assigned", k),
                    });
                }
            }
        }
        Ok(None)
    }

    pub fn resolve(&mut self, stmts: &Vec<Stmt>) -> Result<Option<Value>> {
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

    fn resolve_local(&mut self, expr: &Expr, name: &str, assign: bool) -> Result<Option<Value>> {
        trace!("resolving to locals: {} with stack: {}", expr, self,);
        for (idx, scope) in self.stack.iter_mut().rev().enumerate() {
            trace!(
                "searching for {} within stack no: {} and curr stack: {:?}",
                expr,
                idx,
                &scope
            );
            if scope.contains_key(name) {
                trace!("found! resolving {} within stack no.: {}", expr, idx,);
                self.interpreter.as_ref().borrow_mut().resolve(expr, idx);
                if assign {
                    trace!("Also setting {} to assigned", name);
                    scope.insert(name.to_owned(), VarStatus::Assigned);
                }
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
        match stmt {
            Stmt::Print(stmt) => self.print_stmt(stmt),
            Stmt::Class(stmt) => self.class_stmt(stmt),
            Stmt::Return(stmt) => self.return_stmt(stmt),
            Stmt::Expr(stmt) => self.eval_stmt(stmt),
            Stmt::Fun(stmt) => self.fun_stmt(stmt),
            Stmt::Var(stmt) => self.var_stmt(stmt),
            Stmt::Block(stmt) => self.block_stmt(stmt, Rc::new(Scope::new())),
            Stmt::If(stmt) => self.if_stmt(stmt),
            Stmt::While(stmt) => self.while_stmt(stmt),
        }
    }

    fn print_stmt(&mut self, stmt: &StmtPrint) -> Result<Option<Value>> {
        self.resolve_expr(&stmt.expr)
    }

    fn eval_stmt(&mut self, stmt: &StmtExpr) -> Result<Option<Value>> {
        self.resolve_expr(&stmt.expr)
    }

    fn return_stmt(&mut self, stmt: &StmtReturn) -> Result<Option<Value>> {
        match self.curr_function {
            FuncType::None => Err(LoxErr::Resolve {
                message: format!(
                    "Can't return from non-function scope\n at line: {}, col: {}",
                    stmt.keyword.line, stmt.keyword.column,
                ),
            }),
            FuncType::Function => self.resolve_expr(&stmt.val),
        }
    }

    fn var_stmt(&mut self, var: &StmtVar) -> Result<Option<Value>> {
        self.declare(&var.token)?;
        if let Some(expr) = &var.expr {
            self.resolve_expr(expr)?;
            self.define(&var.token)?;
            self.assign(&var.token)?;

            return Ok(None);
        }

        self.define(&var.token)?;
        Ok(None)
    }

    fn fun_stmt(&mut self, stmt: &StmtFun) -> Result<Option<Value>> {
        self.declare(&stmt.name)?;
        self.define(&stmt.name)?;
        self.assign(&stmt.name)?;
        self.resolve_fun_stmt(stmt)
    }

    fn block_stmt(
        &mut self,
        block: &StmtBlock,
        _scope: Rc<loxrs_env::Scope<Value>>,
    ) -> Result<Option<Value>> {
        trace!(
            "creating stack from block stmt: {} with stack: {}",
            block,
            self,
        );
        self.begin_scope();

        trace!("traversing block with this new stack: {}", &self);
        self.resolve(&block.stmts)?;

        trace!("done traversing block! ejecting scope... {}", &self);
        self.end_scope()?;

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

    fn class_stmt(&mut self, stmt: &StmtClass) -> Result<Option<Value>> {
        self.declare(&stmt.name)?;
        self.define(&stmt.name)?;
        self.assign(&stmt.name)?;
        Ok(None)
    }
}

impl ExprVisitor<Option<Value>> for Resolver {
    fn func(&mut self, def: &ExprFunction) -> Result<Option<Value>> {
        let prev_function_type = self.curr_function.clone();
        self.curr_function = FuncType::Function;

        self.begin_scope();

        for param in &def.params {
            self.declare(param)?;
            self.define(param)?;
            self.assign(param)?;
        }

        self.resolve_stmt(&Stmt::Block(StmtBlock {
            stmts: def.body.stmts.to_owned(),
        }))?;

        self.end_scope()?;

        self.curr_function = prev_function_type;
        Ok(None)
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

    fn var(&mut self, expression: &Expr) -> Result<Option<Value>> {
        if let ExprKind::Var(var) = &expression.kind {
            trace!("var expr: {}", var);
            let name = var.extract_identifier_str()?;
            if self
                .stack
                .last()
                .is_some_and(|last| last.get(name).is_some_and(|el| *el == VarStatus::Declared))
            {
                return Err(LoxErr::Resolve {
                    message: format!("Can't read local variable {} from its own initalizer\n at line: {}, col: {}", name, var.line, 
                        var.column),
                });
            }

            trace!("resolving to locals from var expr: {}", var);
            self.resolve_local(expression, name, false)?;
            return Ok(None);
        }

        Err(LoxErr::Internal {
            message: format!(
                "{} not expected in `var` code path, programmer error",
                expression
            ),
        })
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

    fn assign(&mut self, expr: &Expr) -> Result<Option<Value>> {
        if let ExprKind::Assign(expr_assign) = &expr.kind {
            trace!(
                "assign expr: name: {}, expr: {}",
                expr_assign.name,
                expr_assign.expression
            );
            self.resolve_expr(&expr_assign.expression)?;
            trace!(
                "resolving to locals from assign expr...\nname: Token: {}, expr: {}",
                expr_assign.name,
                expr
            );
            // TODO need to set assigned here too
            return self.resolve_local(expr, expr_assign.name.extract_identifier_str()?, true);
        }

        Err(LoxErr::Internal {
            message: format!(
                "{} not expected in `assign` code path, programmer error",
                expr
            ),
        })
    }
}
