use std::rc::Rc;

use crate::token::Token;

use super::function::FunctionStmt;

#[derive(Clone, Debug)]
pub struct ClassStmt {
    pub name: Token,
    pub methods: Vec<Rc<FunctionStmt>>,
}