use std::fmt::{self, Display};
use std::hash::Hash;

use uuid::Uuid;

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
pub struct Expr {
    id: Uuid,
    pub kind: ExprKind,
}

impl Eq for Expr {}

impl Expr {
    pub fn new(kind: ExprKind) -> Self {
        Self {
            id: Uuid::new_v4(),
            kind,
        }
    }
}

impl Hash for Expr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Expr<[id: {}], [kind: {}]>", self.id, self.kind)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Unary(Box<ExprUnary>),
    Binary(Box<ExprBinary>),
    Call(Box<ExprCall>),
    Logical(Box<ExprLogical>),
    Literal(Box<Literal>),
    Grouping(Box<ExprGrouping>),
    Function(Box<ExprFunction>),
    Var(Token),
    Assign(Box<ExprAssign>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprFunction {
    pub params: Vec<Token>,
    pub body: StmtBlock,
}

impl Display for ExprFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("[ExprFunction]")
            .field("params", &self.params)
            .field("body", &self.body)
            .finish()

        // let params: Vec<&str> = self
        //     .params
        //     .iter()
        //     .map(|el| el.extract_identifier_str().unwrap())
        //     .collect();
        //
        // write!(f, "Expr[Function] ")?;
        // if !params.is_empty() {
        //     write!(f, "params: [{:?}] ", params)?;
        // }
        //
        // write!(f, "body:\n  {}", self.body)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprCall {
    pub callee: Expr,
    pub paren: Token,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprAssign {
    pub name: Token,
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

impl fmt::Display for ExprKind {
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
                Self::Var(var) => var.literal.clone().map_or("None".to_string(), |t| format!(
                    "line: {}, col: {}, {}",
                    var.line, var.column, t
                )
                .to_string()),
                Self::Assign(assign) =>
                    format!("target: {}, expr: {}", assign.name, assign.expression),
            }
        )
    }
}
