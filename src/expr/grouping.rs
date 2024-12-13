use super::Expr;

#[derive(Clone, Debug)]
pub struct Grouping {
    pub expression: Expr,
}
impl From<Expr> for Grouping {
    fn from(expression: Expr) -> Self {
        Self { expression }
    }
}
