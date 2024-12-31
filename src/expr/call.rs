use crate::token::Token;

use super::Expr;

#[derive(Clone, Debug)]
pub struct CallExpr {
    pub callee: Expr,
    pub arguments: Vec<Expr>,
    pub parent: Token,
}
impl CallExpr {}
