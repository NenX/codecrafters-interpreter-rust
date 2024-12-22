use crate::expr::Expr;


#[derive(Clone, Debug)]

pub struct ExpressionStmt {
    pub expression: Expr,
}
