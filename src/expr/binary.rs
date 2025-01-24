use crate::token::Token;

use super::Expr;

#[derive(Clone, Debug)]
pub struct BinaryExpr {
    pub left: Expr,
    pub right: Expr,
    pub operator: Token,
}
