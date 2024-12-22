use crate::expr::Expr;


#[derive(Clone, Debug)]

pub struct PrintStmt {
    pub expression: Expr,
}
