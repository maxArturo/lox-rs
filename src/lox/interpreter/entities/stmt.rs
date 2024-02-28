use super::Expr;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
}
