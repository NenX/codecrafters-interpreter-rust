use crate::{expr::Expr, token::Token};

use super::Stmt;

#[derive(Clone, Debug)]

pub struct FunctionStmt {
    pub name:Token,
    pub params: Vec<Token>,
    pub body: Stmt,
}

impl FunctionStmt {

}
