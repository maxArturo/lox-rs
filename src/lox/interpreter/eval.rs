use super::{
    entities::{Expr, Literal, Stmt, Token, TokenType, Value},
    error::{LoxErr, Result},
};

#[derive(Debug)]
pub struct Interpreter;

impl Interpreter {
    pub fn interpret(&self, stmts: &[Stmt]) -> Result<()> {
        for s in stmts {
            self.exec_stmt(s)?;
        }
        Ok(())
    }

    fn truthy(&self, val: Value) -> Result<Value> {
        Ok(Value::Boolean(match val {
            Value::Boolean(val) => val,
            Value::Nil => false,
            _ => true,
        }))
    }

    fn error(&self, expr: Vec<&Expr>, message: Option<&str>) -> LoxErr {
        let err = LoxErr::Eval {
            expr: expr
                .iter()
                .map(|el| el.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            message: message
                .unwrap_or("Expression evaluation failed")
                .to_string(),
        };
        eprintln!("{}", err);
        err
    }
}

impl ExprEval<Value> for Interpreter {
    fn literal(&self, literal: &Literal) -> Result<Value> {
        Ok(literal.clone())
    }

    fn unary(&self, right: &Expr, operator: &Token) -> Result<Value> {
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
            TokenType::Bang => self.truthy(eval_right),
            _ => err_report(Some(&format!(
                "Unexpected token in unary expr: `{}`",
                operator.token_type
            ))),
        }
    }

    fn binary(&self, left: &Expr, right: &Expr, operator: &Token) -> Result<Value> {
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

    fn grouping(&self, expression: &Expr) -> Result<Value> {
        self.eval(expression)
    }

    fn var(&self, expression: &Token) -> Result<Value> {
        todo!()
    }
}

impl StmtExec<()> for Interpreter {
    fn print_stmt(&self, expr: &Expr) -> Result<()> {
        let val = self.eval(expr)?;
        println!("{}", val);
        Ok(())
    }

    fn eval_stmt(&self, expr: &Expr) -> Result<()> {
        self.eval(expr)?;
        Ok(())
    }

    fn var_stmt(&self, token: &Token, expr: &Option<Expr>) -> Result<()> {
        todo!()
    }
}

trait StmtExec<T> {
    fn exec_stmt(&self, stmt: &Stmt) -> Result<T> {
        match stmt {
            Stmt::Print(expr) => self.print_stmt(expr),
            Stmt::Expr(expr) => self.eval_stmt(expr),
            Stmt::Var(token, expr) => self.var_stmt(token, expr),
        }
    }
    fn print_stmt(&self, expr: &Expr) -> Result<T>;
    fn eval_stmt(&self, expr: &Expr) -> Result<T>;
    fn var_stmt(&self, token: &Token, expr: &Option<Expr>) -> Result<T>;
}

trait ExprEval<T> {
    fn eval(&self, expr: &Expr) -> Result<T> {
        match expr {
            Expr::Unary(unary) => self.unary(&unary.right, &unary.operator),
            Expr::Binary(binary) => self.binary(&binary.left, &binary.right, &binary.operator),
            Expr::Grouping(grouping) => self.grouping(&grouping.expression),
            Expr::Literal(lit) => self.literal(lit),
            Expr::Var(var) => self.var(var),
        }
    }
    fn literal(&self, literal: &Literal) -> Result<T>;
    fn unary(&self, right: &Expr, operator: &Token) -> Result<T>;
    fn binary(&self, left: &Expr, right: &Expr, operator: &Token) -> Result<T>;
    fn grouping(&self, expression: &Expr) -> Result<T>;
    fn var(&self, expression: &Token) -> Result<T>;
}
