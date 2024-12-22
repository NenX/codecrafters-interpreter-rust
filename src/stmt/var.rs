use crate::{expr::Expr, token::Token};

#[derive(Clone, Debug)]

pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Expr>,
}
