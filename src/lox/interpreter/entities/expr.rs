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

#[derive(Debug)]
pub enum Expr {
    Unary {
        right: Box<Expr>,
        operator: Token,
    },
    Binary {
        left: Box<Expr>,
        right: Box<Expr>,
        operator: Token,
    },
    Literal(Literal),
    Grouping {
        expression: Box<Expr>,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Grouping { expression } => parenthesize("grouping", vec![expression]),
                Self::Unary { right, operator } =>
                    parenthesize(&operator.token_type.to_string(), vec![right]),
                Self::Binary {
                    left,
                    right,
                    operator,
                } => parenthesize(&operator.token_type.to_string(), vec![left, right]),
                Self::Literal(value) => value.to_string(),
            }
        )
    }
}
