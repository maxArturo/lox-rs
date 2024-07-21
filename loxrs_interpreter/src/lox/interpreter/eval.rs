use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};
use std::vec;

use log::{debug, trace};

use crate::lox::entities::expr::{ExprFunction, ExprKind};
use crate::lox::entities::func::Func;
use crate::lox::entities::stmt::StmtClass;
use crate::lox::entities::Class;

use super::super::entities::eval::Interpreter;
use super::super::entities::func::{Function, NativeFunction};
use super::super::entities::stmt::{StmtFun, StmtReturn};
use super::super::entities::{
    expr::ExprGrouping,
    stmt::{StmtBlock, StmtExpr, StmtIf, StmtPrint, StmtVar, StmtWhile},
    Expr, Literal, Stmt, Token, TokenType, Value,
};
use super::visitor::{ExprVisitor, StmtVisitor};

use loxrs_env::Scope;
use loxrs_types::{LoxErr, Result};

impl Interpreter {
    pub fn new() -> Self {
        let scope = Self::setup_native_fns();
        Self {
            scope: Rc::clone(&scope),
            globals: Rc::clone(&scope),
            locals: RefCell::new(HashMap::new()),
        }
    }

    fn setup_native_fns() -> Rc<Scope<Value>> {
        let scope = Rc::new(Scope::new());

        scope.define(
            "clock",
            Value::Func(Func::Native(NativeFunction::new(
                |_args, _env| {
                    Ok(Value::Number(
                        match SystemTime::now().duration_since(UNIX_EPOCH) {
                            Ok(n) => n.as_secs_f64(),
                            Err(_) => panic!("system `time` before UNIX EPOCH!"),
                        },
                    ))
                },
                Rc::clone(&scope),
                &[],
                "time",
            ))),
        );
        scope
    }

    pub fn interpret(&mut self, stmts: &[Stmt]) -> Result<()> {
        for s in stmts {
            trace!("evaluating statement:\n{:#?}", s);
            self.exec_stmt(s)?;
        }
        Ok(())
    }

    pub fn scope(&self) -> Rc<Scope<Value>> {
        Rc::clone(&self.scope)
    }

    fn lookup_var(&self, name: &str, expr: &Expr) -> Result<Value> {
        trace!(
            "[lookup_var] looking up {} in {:?}",
            expr,
            self.locals.borrow()
        );

        if let Some(distance) = self.locals.borrow().get(expr) {
            trace!("[lookup_var] found {} with value of {}", expr, distance);
            return self.scope.get_at(*distance, name);
        }
        self.globals.get(name)
    }

    fn truthy(&self, val: &Value) -> Value {
        Value::Boolean(match val {
            &Value::Boolean(val) => val,
            Value::Nil => false,
            _ => true,
        })
    }

    fn error(&self, expr: Vec<&Expr>, message: Option<&str>) -> LoxErr {
        LoxErr::Eval {
            expr: expr
                .iter()
                .map(|el| el.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            message: message
                .unwrap_or("Expression evaluation failed")
                .to_string(),
        }
    }
}

impl ExprVisitor<Value> for Interpreter {
    fn func(&mut self, def: &ExprFunction) -> Result<Value> {
        let scope = self.scope();

        let func = Value::Func(Func::Lox(Function {
            def: def.clone(),
            scope,
            params: def.params.clone(),
        }));

        Ok(func)
    }

    fn literal(&mut self, literal: &Literal) -> Result<Value> {
        Ok(literal.clone())
    }

    fn unary(&mut self, right: &Expr, operator: &Token) -> Result<Value> {
        let eval_right: Value = self.eval(right)?;
        let err_report = |other: Option<&str>| Err(self.error(vec![right], other));

        match operator.token_type {
            TokenType::Minus => match eval_right {
                Value::Number(num) => Ok(Value::Number(-num)),
                _ => err_report(Some(&format!(
                    "Unexpected value in unary expr for `-`: {}",
                    eval_right
                ))),
            },
            TokenType::Bang => Ok(self.truthy(&eval_right)),
            _ => err_report(Some(&format!(
                "Unexpected token in unary expr: `{}`",
                operator.token_type
            ))),
        }
    }

    fn binary(&mut self, left: &Expr, right: &Expr, operator: &Token) -> Result<Value> {
        let left_val = self.eval(left)?;
        let right_val = self.eval(right)?;

        let err_report = |reason: Option<&str>| {
            Err(self.error(
                vec![left, right],
                reason.or(Some(&format!(
                    "Unexpected values in binary expr: {:#?}",
                    (&left_val, &right_val)
                ))),
            ))
        };

        let bin_err = || err_report(None);

        match operator.token_type {
            // `TokenType::Plus` is overloaded for both arithmatic and string concat
            TokenType::Plus => match (&left_val, &right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                (Value::String(l), Value::String(r)) => Ok(Value::String(l.to_string() + r)),
                (Value::String(l), Value::Number(r)) => {
                    Ok(Value::String(l.to_string() + &r.to_string()))
                }
                (Value::Number(l), Value::String(r)) => Ok(Value::String(l.to_string() + &r)),
                _ => err_report(Some(&format!(
                    "unexpected types for addition operation: {:#?}",
                    (&left_val, &right_val)
                ))),
            },
            TokenType::Minus => match (&left_val, &right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
                _ => bin_err(),
            },
            TokenType::Slash => match (&left_val, &right_val) {
                (Value::Number(l), Value::Number(r)) => {
                    if *l == 0.0f64 || *r == 0.0f64 {
                        return err_report(Some("Division by zero"));
                    }
                    Ok(Value::Number(l / r))
                }
                _ => bin_err(),
            },
            TokenType::Star => match (&left_val, &right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
                _ => bin_err(),
            },
            TokenType::Greater => match (&left_val, &right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l > r)),
                _ => bin_err(),
            },
            TokenType::GreaterEqual => match (&left_val, &right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l >= r)),
                _ => bin_err(),
            },
            TokenType::Less => match (&left_val, &right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l < r)),
                _ => bin_err(),
            },
            TokenType::LessEqual => match (&left_val, &right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l <= r)),
                _ => bin_err(),
            },
            TokenType::BangEqual => Ok(Value::Boolean(left_val != right_val)),
            TokenType::EqualEqual => Ok(Value::Boolean(left_val == right_val)),
            _ => bin_err(),
        }
    }

    fn grouping(&mut self, expression: &ExprGrouping) -> Result<Value> {
        self.eval(&expression.expression)
    }

    fn var(&mut self, expression: &Expr) -> Result<Value> {
        if let ExprKind::Var(var) = &expression.kind {
            let str = var.extract_identifier_str()?;

            let res = self.lookup_var(str, expression)?;
            return Ok(res.clone());
        }

        Err(LoxErr::Internal {
            message: format!(
                "{} not expected in `var` code path, programmer error",
                expression
            ),
        })
    }

    fn assign(&mut self, expr: &Expr) -> Result<Value> {
        if let ExprKind::Assign(expr_assign) = &expr.kind {
            let val = self.eval(&expr_assign.expression)?;
            let var_name = expr_assign.name.extract_identifier_str()?;

            if let Ok(Value::Func(Func::Native(_func))) = self.globals.get(var_name) {
                // TODO missing tests here
                return Err(LoxErr::Eval {
                    expr: var_name.to_owned(),
                    message: "Not allowed to override native function".to_owned(),
                });
            }

            if let Some(distance) = self.locals.borrow().get(expr) {
                self.scope.assign_at(*distance, var_name, val.clone())?;
            } else {
                self.globals.define(var_name, val.clone());
            }

            return Ok(val);
        }

        Err(LoxErr::Internal {
            message: format!(
                "{} not expected in `assign` code path, programmer error",
                expr
            ),
        })
    }

    fn logical(&mut self, left: &Expr, right: &Expr, operator: &Token) -> Result<Value> {
        let left_val = self.eval(left)?;
        let left_truthy = self.truthy(&left_val);

        match operator.token_type {
            TokenType::And => {
                if let Value::Boolean(true) = left_truthy {
                    return self.eval(right);
                }
                Ok(Value::Boolean(false))
            }
            TokenType::Or => {
                if let Value::Boolean(true) = left_truthy {
                    return Ok(left_val);
                }
                self.eval(right)
            }
            _ => Err(self.error(
                vec![left, right],
                Some(&format!(
                    "Unexpected token type in logic expr: {:#?}",
                    operator
                )),
            )),
        }
    }

    fn call(&mut self, callee: &Expr, args: &[Expr]) -> Result<Value> {
        let mut fun = match self.eval(callee)? {
            Literal::Func(val) => val,
            _ => {
                return Err(LoxErr::Eval {
                    expr: callee.to_string(),
                    message: "Invalid call on non-func value".to_string(),
                })
            }
        };

        if fun.arity() != args.len() {
            return Err(LoxErr::Eval {
                expr: format!("{:?}", args),
                message: format!("Expected {} args but got {}", fun.arity(), args.len())
                    .to_string(),
            });
        }

        let mut args_eval = vec![];

        for arg in args {
            args_eval.push(self.eval(arg)?);
        }

        fun.call(self, args_eval)
    }

    fn get(&mut self, name: &Token, expr: &Expr) -> Result<Value> {
        match self.eval(expr)? {
            Literal::Instance(instance) => {
                trace!("getting {} from {}", name, instance);
                instance.get(name.extract_identifier_str()?)
            }
            _ => Err(LoxErr::Eval {
                expr: expr.to_string(),
                message: "Invalid call on non-instance value".to_string(),
            }),
        }
    }

    fn set(&mut self, name: &Token, target: &Expr, value: &Expr) -> Result<Value> {
        match self.eval(target)? {
            Literal::Instance(instance) => {
                let val = self.eval(value)?;

                trace!("setting field: {}\nto: {}\non: {}", name, val, instance);
                instance.set(name.extract_identifier_str()?, val.to_owned());
                trace!(
                    "setting field Complete: {}\nto: {}\non: {}",
                    name,
                    val,
                    instance
                );

                trace!("curr scope: {}", &self.scope);
                Ok(val)
            }
            _ => Err(LoxErr::Eval {
                expr: target.to_string(),
                message: "Only instances can be accessed via fields (`.`)".to_string(),
            }),
        }
    }
}

impl StmtVisitor for Interpreter {
    fn exec_stmt(&mut self, stmt: &Stmt) -> Result<Option<Value>> {
        let res = match stmt {
            Stmt::Print(stmt) => self.print_stmt(stmt),
            Stmt::Class(stmt) => self.class_stmt(stmt),
            Stmt::Return(stmt) => self.return_stmt(stmt),
            Stmt::Expr(stmt) => self.eval_stmt(stmt),
            Stmt::Fun(stmt) => self.fun_stmt(stmt),
            Stmt::Var(stmt) => self.var_stmt(stmt),
            Stmt::Block(stmt) => self.block_stmt(stmt, self.scope()),
            Stmt::If(stmt) => self.if_stmt(stmt),
            Stmt::While(stmt) => self.while_stmt(stmt),
        };
        debug!("statement execution result for {}: {:?}", stmt, res);
        res
    }

    fn print_stmt(&mut self, stmt: &StmtPrint) -> Result<Option<Value>> {
        let val = self.eval(&stmt.expr)?;
        debug!("the returned value is: {val}");
        println!("==> {}", val);
        Ok(None)
    }

    fn eval_stmt(&mut self, stmt: &StmtExpr) -> Result<Option<Value>> {
        trace!("eval statement: {:?}", stmt);
        self.eval(&stmt.expr)?;
        Ok(None)
    }

    fn return_stmt(&mut self, stmt: &StmtReturn) -> Result<Option<Value>> {
        let res = self.eval(&stmt.val).map(Some);
        trace!("returning from statement with value: {:?}", res);
        res
    }

    fn var_stmt(&mut self, var: &StmtVar) -> Result<Option<Value>> {
        let val = var.expr.as_ref().map_or(Ok(Value::Nil), |e| self.eval(e))?;
        self.scope.define(var.token.extract_identifier_str()?, val);

        trace!("var statement {:?} defined in scope: {}", var, self.scope);
        Ok(None)
    }

    fn fun_stmt(&mut self, stmt: &StmtFun) -> Result<Option<Value>> {
        trace!("assigning the following env to {:?}: {}", stmt, &self.scope);

        let func = self.func(&stmt.def)?;

        self.scope
            .define(stmt.name.extract_identifier_str()?, func.clone());

        Ok(None)
    }

    fn block_stmt(&mut self, block: &StmtBlock, scope: Rc<Scope<Value>>) -> Result<Option<Value>> {
        let new_scope = Scope::from_parent(scope);

        trace!("block statement: {:?}", block);
        let prev_scope = Rc::clone(&self.scope);

        self.scope = new_scope;
        trace!("new scope for block statement: {}", self.scope);

        let mut res = Ok(None);

        for stmt in block.stmts.as_slice() {
            if let Some(val) = self.exec_stmt(stmt)? {
                res = Ok(Some(val));
                break;
            }
        }

        self.scope = prev_scope;
        res
    }

    fn if_stmt(&mut self, stmt: &StmtIf) -> Result<Option<Value>> {
        let res = self.eval(&stmt.cond)?;
        if let Literal::Boolean(true) = self.truthy(&res) {
            trace!("entering `then` side of if: {:?}", stmt.then);
            return self.exec_stmt(&stmt.then);
        }

        if let Some(other) = &stmt.else_stmt {
            trace!("entering `else` side of if: {:?}", stmt.else_stmt);
            return self.exec_stmt(other);
        }
        Ok(None)
    }

    fn while_stmt(&mut self, stmt: &StmtWhile) -> Result<Option<Value>> {
        trace!("while statement: {:?}", stmt);
        let mut res = self.eval(&stmt.expr)?;

        while let Literal::Boolean(true) = self.truthy(&res) {
            self.exec_stmt(&stmt.stmt)?;
            res = self.eval(&stmt.expr)?;
        }
        Ok(None)
    }

    fn class_stmt(&mut self, stmt: &StmtClass) -> Result<Option<Value>> {
        let name = stmt.name.extract_identifier_str()?;
        self.scope.define(name, Value::Nil);
        // let class = ;

        self.scope.assign(
            name,
            Value::Func(Func::Class(Rc::new(Class {
                name: name.to_owned(),
                methods: Vec::new(),
            }))),
        )?;
        Ok(None)
    }
}
