use crate::token::Token;

use super::Stmt;

#[derive(Clone, Debug)]

pub struct FunctionStmt {
    pub name:Token,
    pub params: Vec<Token>,
    pub fn_body: Vec<Stmt>,
}

impl FunctionStmt {

}
