use std::{fmt, ops::Deref};

use super::Token;

fn parenthesize(name: &str, expressions: Vec<&Expr>) -> String {
    String::from("(")
        + name
        + " "
        + &expressions
            .iter()
            .map(|el| el.pretty_print())
            .collect::<Vec<String>>()
            .join(" ")
        + ")"
}

pub enum Expr {
    Grouping {
        expression: Box<Expr>,
    },
    Unary {
        right: Box<Expr>,
        operator: Token,
    },
    Binary {
        left: Box<Expr>,
        right: Box<Expr>,
        operator: Token,
    },
    Literal {
        value: Token,
    },
}

impl Expr {
    fn pretty_print(&self) -> String {
        match self {
            Self::Grouping { expression } => parenthesize("grouping", vec![expression.deref()]),
            Self::Unary { right, operator } => parenthesize(&operator.lexeme, vec![right.deref()]),
            Self::Binary {
                left,
                right,
                operator,
            } => parenthesize(&operator.lexeme, vec![left.deref(), right.deref()]),
            Self::Literal { value } => String::from(&value.lexeme),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.pretty_print())
    }
}
