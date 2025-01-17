use crate::token::Token;

use super::Expr;

#[derive(Clone, Debug)]
pub struct UnaryExpr {
    pub right: Expr,
    pub operator: Token,
}
