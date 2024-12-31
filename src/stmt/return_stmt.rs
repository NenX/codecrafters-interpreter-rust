use crate::{expr::Expr, token::Token};

#[derive(Clone, Debug)]
pub struct ReturnStmt {
    pub keyword: Token,
    pub value: Option<Expr>,
}

impl ReturnStmt {}
