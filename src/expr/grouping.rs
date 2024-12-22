use super::Expr;

#[derive(Clone, Debug)]
pub struct GroupingExpr {
    pub expression: Expr,
}
impl From<Expr> for GroupingExpr {
    fn from(expression: Expr) -> Self {
        Self { expression }
    }
}
