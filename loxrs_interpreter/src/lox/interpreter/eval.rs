use std::cell::RefCell;
use std::time::SystemTime;
use std::vec;

use log::debug;

use crate::lox::entities::func::Func;

use super::super::entities::eval::Interpreter;
use super::super::entities::func::{Function, NativeFunction};
use super::super::entities::stmt::{StmtFun, StmtReturn};
use super::super::entities::{
    expr::{ExprAssign, ExprGrouping},
    stmt::{StmtBlock, StmtExpr, StmtIf, StmtPrint, StmtVar, StmtWhile},
    Expr, Literal, Stmt, Token, TokenType, Value,
};

use loxrs_env::Env;
use loxrs_types::{LoxErr, Result};

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Self::setup_native_fns(),
        }
    }

    fn setup_native_fns() -> RefCell<Env<Value>> {
        let env: RefCell<Env<Value>> = RefCell::new(Env::default());

        let fn_env = env.clone();
        env.borrow_mut().define_global(
            "time",
            Value::Func(Func::Native(NativeFunction::new(
                |_args, _env| Ok(Value::String(format!("{:#?}", SystemTime::now()))),
                fn_env,
                &[],
                "time",
            ))),
        );
        env
    }

    pub fn interpret(&mut self, stmts: &[Stmt]) -> Result<()> {
        for s in stmts {
            self.exec_stmt(s)?;
        }
        Ok(())
    }

    pub fn env(&self) -> RefCell<Env<Value>> {
        let env = self.env.clone();
        env.borrow_mut().open_scope();
        env
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

impl Interpreter {
    fn eval(&mut self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Unary(unary) => self.unary(&unary.right, &unary.operator),
            Expr::Binary(binary) => self.binary(&binary.left, &binary.right, &binary.operator),
            Expr::Logical(logical) => {
                self.logical(&logical.left, &logical.right, &logical.operator)
            }
            Expr::Grouping(grouping) => self.grouping(grouping),
            Expr::Literal(lit) => self.literal(lit),
            Expr::Var(var) => self.var(var),
            Expr::Assign(token, expr) => self.assign(token, expr),
            Expr::Call(call) => self.call(&call.callee, &call.args),
        }
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

    fn var(&self, expression: &Token) -> Result<Value> {
        let str = expression.extract_identifier_str()?;
        let res = self.env.borrow().get(str)?;
        Ok(res.clone())
    }

    fn assign(&mut self, token: &Token, expr: &ExprAssign) -> Result<Value> {
        let val = self.eval(&expr.expression)?;
        self.env
            .borrow_mut()
            .assign(token.extract_identifier_str()?, val.clone())?;
        Ok(val)
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
                    message: "Invalid call".to_string(),
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
}

impl Interpreter {
    fn exec_stmt(&mut self, stmt: &Stmt) -> Result<Option<Value>> {
        let res = match stmt {
            Stmt::Print(stmt) => self.print_stmt(stmt),
            Stmt::Return(stmt) => self.return_stmt(stmt),
            Stmt::Expr(stmt) => self.eval_stmt(stmt),
            Stmt::Fun(stmt) => self.fun_stmt(stmt),
            Stmt::Var(stmt) => self.var_stmt(stmt),
            Stmt::Block(stmt) => self.block_stmt(stmt, self.env()),
            Stmt::If(stmt) => self.if_stmt(stmt),
            Stmt::While(stmt) => self.while_stmt(stmt),
        };
        debug!("statement execution result: {:?}", res);
        res
    }

    fn print_stmt(&mut self, stmt: &StmtPrint) -> Result<Option<Value>> {
        let val = self.eval(&stmt.expr)?;
        debug!("the returned value is: {val}");
        println!("==> {}", val);
        Ok(None)
    }

    fn eval_stmt(&mut self, stmt: &StmtExpr) -> Result<Option<Value>> {
        self.eval(&stmt.expr).map(Some)
    }

    fn return_stmt(&mut self, stmt: &StmtReturn) -> Result<Option<Value>> {
        self.eval(&stmt.val).map(Some)
    }

    fn var_stmt(&mut self, var: &StmtVar) -> Result<Option<Value>> {
        let val = var.expr.as_ref().map_or(Ok(Value::Nil), |e| self.eval(e))?;
        self.env
            .borrow_mut()
            .define(var.token.extract_identifier_str()?, val);
        Ok(None)
    }

    fn fun_stmt(&mut self, def: &StmtFun) -> Result<Option<Value>> {
        self.env.borrow_mut().define(
            def.name.extract_identifier_str()?,
            Value::Func(Func::Lox(Function {
                def: Box::new(def.clone()),
                env: self.env(),
            })),
        );

        Ok(None)
    }

    pub fn block_stmt(
        &mut self,
        block: &StmtBlock,
        env: RefCell<Env<Value>>,
    ) -> Result<Option<Value>> {
        let prev_env = RefCell::clone(&self.env);
        self.env = env;

        debug!("curr env block status: {:?}", self.env);
        for stmt in block.stmts.as_slice() {
            if let Stmt::Return(s) = stmt {
                return self.return_stmt(s);
            }
            self.exec_stmt(stmt)?;
        }

        self.env = prev_env;
        Ok(None)
    }

    fn if_stmt(&mut self, stmt: &StmtIf) -> Result<Option<Value>> {
        let res = self.eval(&stmt.cond)?;
        if let Literal::Boolean(true) = self.truthy(&res) {
            return self.exec_stmt(&stmt.then);
        }
        if let Some(other) = &stmt.else_stmt {
            return self.exec_stmt(other);
        }
        Ok(None)
    }

    fn while_stmt(&mut self, stmt: &StmtWhile) -> Result<Option<Value>> {
        let mut res = self.eval(&stmt.expr)?;

        while let Literal::Boolean(true) = self.truthy(&res) {
            if let Stmt::Return(s) = stmt.stmt.as_ref() {
                return self.return_stmt(s);
            }
            self.exec_stmt(&stmt.stmt)?;
            res = self.eval(&stmt.expr)?;
        }
        Ok(None)
    }
}