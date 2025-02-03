use crate::token::Token;

use super::Expr;

#[derive(Debug, Clone)]
pub struct GetExpr {
    pub object: Expr,
    pub name: Token,
}