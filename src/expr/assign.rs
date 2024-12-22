use crate::token::Token;

use super::Expr;

#[derive(Clone, Debug)]

pub struct AssignExpr {
    pub name: Token,
    pub value: Expr,
}
