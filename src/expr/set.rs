use crate::token::Token;

use super::Expr;

#[derive(Debug, Clone)]
pub struct SetExpr {
    pub object: Expr,
    pub name: Token,
    pub value: Expr,
}