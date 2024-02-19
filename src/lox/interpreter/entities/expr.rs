use super::Token;

fn parenthesize<T: Expr>(name: &str, expressions: Vec<&T>) -> String {
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

pub trait Expr {
    /// used for representing the tree
    fn pretty_print(&self) -> String;
}

// Grouping expression
#[derive(Debug)]
pub struct Grouping<T: Expr> {
    expression: T,
}

impl<T: Expr> Expr for Grouping<T> {
    fn pretty_print(&self) -> String {
        self.expression.pretty_print()
    }
}

impl<T: Expr> Grouping<T> {
    pub fn new(expression: T) -> Self {
        Grouping { expression }
    }
}

// Unary expression
#[derive(Debug)]
pub struct Unary<T: Expr> {
    right: T,
    operator: Token,
}

impl<T: Expr> Expr for Unary<T> {
    fn pretty_print(&self) -> String {
        parenthesize(&self.operator.lexeme, vec![&self.right])
    }
}

impl<T: Expr> Unary<T> {
    pub fn new(right: T, operator: Token) -> Self {
        Unary { right, operator }
    }
}

// Binary expression
#[derive(Debug)]
pub struct Binary<T: Expr> {
    left: T,
    right: T,
    operator: Token,
}

impl<T: Expr> Expr for Binary<T> {
    fn pretty_print(&self) -> String {
        parenthesize(&self.operator.lexeme, vec![&self.left, &self.right])
    }
}

impl<T: Expr> Binary<T> {
    pub fn new(left: T, right: T, operator: Token) -> Self {
        Binary {
            left,
            right,
            operator,
        }
    }
}

// Literal expression
#[derive(Debug)]
pub struct Literal {
    value: Token,
}

impl Expr for Literal {
    fn pretty_print(&self) -> String {
        String::from(&self.value.lexeme)
    }
}

impl Literal {
    pub fn new(value: Token) -> Self {
        Literal { value }
    }
}
