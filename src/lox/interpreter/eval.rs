use super::entities::{Expr, Literal, Token, TokenType, Value};

#[derive(Debug)]
struct Interpreter;

impl Interpreter {
    fn truthy(&self, val: Value) -> Value {
        Value::Boolean(match val {
            Value::Boolean(val) => val,
            Value::Nil => false,
            _ => true,
        })
    }
}

impl Eval for Interpreter {
    fn eval_literal(&self, literal: Literal) -> Value {
        literal
    }

    fn eval_unary(&self, right: Box<Expr>, operator: Token) -> Value {
        let eval_right: Value = self.eval(*right);

        match operator.token_type {
            TokenType::Minus => match eval_right {
                Value::Number(num) => Value::Number(-num),
                _ => panic!(
                    "Unexpected value literal of {} in unary expr: {}",
                    eval_right, operator.token_type
                ),
            },
            TokenType::Bang => self.truthy(eval_right),
            _ => panic!("Unexpected token type in unary expr: {}", eval_right),
        }
    }

    fn eval_binary(&self, left: Box<Expr>, right: Box<Expr>, operator: Token) -> Value {
        let left_val = self.eval(*left);
        let right_val = self.eval(*right);

        match operator.token_type {
            // `TokenType::Plus` is overloaded for both arithmatic and string concat
            TokenType::Plus => match (&left_val, &right_val) {
                (Value::Number(l), Value::Number(r)) => Value::Number(l + r),
                (Value::String(l), Value::String(r)) => Value::String(l.to_string() + r),
                _ => panic!(
                    "Unexpected values in binary expr: {:#?}",
                    (&left_val, &right_val)
                ),
            },
            TokenType::Minus => match (&left_val, &right_val) {
                (Value::Number(l), Value::Number(r)) => Value::Number(l - r),
                _ => panic!(
                    "Unexpected values in binary expr: {:#?}",
                    (&left_val, &right_val)
                ),
            },
            TokenType::Slash => match (&left_val, &right_val) {
                (Value::Number(l), Value::Number(r)) => Value::Number(l / r),
                _ => panic!(
                    "Unexpected values in binary expr: {:#?}",
                    (&left_val, &right_val)
                ),
            },
            TokenType::Star => match (&left_val, &right_val) {
                (Value::Number(l), Value::Number(r)) => Value::Number(l * r),
                _ => panic!(
                    "Unexpected values in binary expr: {:#?}",
                    (&left_val, &right_val)
                ),
            },
            TokenType::Greater => match (&left_val, &right_val) {
                (Value::Number(l), Value::Number(r)) => Value::Boolean(l > r),
                _ => panic!(
                    "Unexpected values in binary expr: {:#?}",
                    (&left_val, &right_val)
                ),
            },
            TokenType::GreaterEqual => match (&left_val, &right_val) {
                (Value::Number(l), Value::Number(r)) => Value::Boolean(l >= r),
                _ => panic!(
                    "Unexpected values in binary expr: {:#?}",
                    (&left_val, &right_val)
                ),
            },
            TokenType::Less => match (&left_val, &right_val) {
                (Value::Number(l), Value::Number(r)) => Value::Boolean(l < r),
                _ => panic!(
                    "Unexpected values in binary expr: {:#?}",
                    (&left_val, &right_val)
                ),
            },
            TokenType::LessEqual => match (&left_val, &right_val) {
                (Value::Number(l), Value::Number(r)) => Value::Boolean(l <= r),
                _ => panic!(
                    "Unexpected values in binary expr: {:#?}",
                    (&left_val, &right_val)
                ),
            },
            TokenType::BangEqual => Value::Boolean(left_val != right_val),
            TokenType::EqualEqual => Value::Boolean(left_val == right_val),

            _ => panic!(
                "Unexpected token type in unary expr: {}",
                operator.token_type
            ),
        }
    }

    fn eval_grouping(&self, expression: Box<Expr>) -> Value {
        self.eval(*expression)
    }
    // add code here
}

trait Eval {
    fn eval(&self, expr: Expr) -> Value {
        match expr {
            Expr::Unary { right, operator } => self.eval_unary(right, operator),
            Expr::Binary {
                left,
                right,
                operator,
            } => self.eval_binary(left, right, operator),
            Expr::Grouping { expression } => self.eval_grouping(expression),
            Expr::Literal(lit) => self.eval_literal(lit),
        }
    }
    fn eval_literal(&self, literal: Literal) -> Value;
    fn eval_unary(&self, right: Box<Expr>, operator: Token) -> Value;
    fn eval_binary(&self, left: Box<Expr>, right: Box<Expr>, operator: Token) -> Value;
    fn eval_grouping(&self, expression: Box<Expr>) -> Value;
}
