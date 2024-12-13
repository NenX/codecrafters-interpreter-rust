use crate::token::Token;

use super::Expr;

#[derive(Clone, Debug)]
pub struct Unary {
    pub right: Expr,
    pub operator: Token,
}
