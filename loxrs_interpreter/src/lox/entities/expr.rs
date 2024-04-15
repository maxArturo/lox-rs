use std::fmt::{self, Display};

use super::{stmt::StmtBlock, Literal, Token};

fn parenthesize<T: Display>(name: &str, expressions: Vec<&T>) -> String {
    String::from("(")
        + name
        + " "
        + &expressions
            .iter()
            .map(|el| el.to_string())
            .collect::<Vec<String>>()
            .join(" ")
        + ")"
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Unary(Box<ExprUnary>),
    Binary(Box<ExprBinary>),
    Call(Box<ExprCall>),
    Logical(Box<ExprLogical>),
    Literal(Box<Literal>),
    Grouping(Box<ExprGrouping>),
    Function(Box<ExprFunction>),
    Var(Token),
    Assign(Token, Box<ExprAssign>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprFunction {
    pub params: Vec<Token>,
    pub body: StmtBlock,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprCall {
    pub callee: Expr,
    pub paren: Token,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprAssign {
    pub expression: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprGrouping {
    pub expression: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprUnary {
    pub right: Expr,
    pub operator: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprBinary {
    pub left: Expr,
    pub right: Expr,
    pub operator: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprLogical {
    pub left: Expr,
    pub right: Expr,
    pub operator: Token,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Call(call) => {
                    parenthesize(&call.callee.to_string(), call.args.iter().collect())
                }
                Self::Grouping(grouping) => {
                    parenthesize("grouping", vec![&grouping.expression])
                }
                Self::Function(func) => {
                    parenthesize("<function>", func.params.iter().collect())
                }
                Self::Unary(unary) =>
                    parenthesize(&unary.operator.token_type.to_string(), vec![&unary.right]),
                Self::Binary(binary) => parenthesize(
                    &binary.operator.token_type.to_string(),
                    vec![&binary.left, &binary.right]
                ),
                Self::Logical(logical) => parenthesize(
                    &logical.operator.token_type.to_string(),
                    vec![&logical.left, &logical.right]
                ),
                Self::Literal(value) => value.to_string(),
                Self::Var(var) => var
                    .literal
                    .clone()
                    .map_or("None".to_string(), |t| t.to_string()),
                Self::Assign(token, expr) => format!("token: {}, expr: {}", token, expr.expression),
            }
        )
    }
}
