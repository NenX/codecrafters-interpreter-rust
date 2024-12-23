use crate::expr::Expr;


#[derive(Clone, Debug)]

pub struct ExpressionStmt {
    pub expression: Expr,
}
impl From<Expr> for ExpressionStmt {
    fn from(expression: Expr) -> Self {
       Self { expression }
    }
}