use crate::token::Token;

use super::Expr;

#[derive(Clone, Debug)]
pub struct LogicalExpr {
    pub left: Expr,
    pub right: Expr,
    pub operator: Token,
}
