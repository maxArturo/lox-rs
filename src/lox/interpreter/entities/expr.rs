use std::fmt;

use super::{Literal, Token};

fn parenthesize(name: &str, expressions: Vec<&Expr>) -> String {
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
    Literal(Literal),
    Grouping(Box<ExprGrouping>),
    Var(Token),
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

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Grouping(grouping) => {
                    parenthesize("grouping", vec![&grouping.expression])
                }
                Self::Unary(unary) =>
                    parenthesize(&unary.operator.token_type.to_string(), vec![&unary.right]),
                Self::Binary(binary) => parenthesize(
                    &binary.operator.token_type.to_string(),
                    vec![&binary.left, &binary.right]
                ),
                Self::Literal(value) => value.to_string(),
                Self::Var(var) => var
                    .literal
                    .clone()
                    .map_or("None".to_string(), |t| t.to_string()),
            }
        )
    }
}
