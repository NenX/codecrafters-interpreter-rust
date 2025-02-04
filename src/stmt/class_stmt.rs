use std::rc::Rc;


use crate::{
    expr::Expr,
    token::Token,
};

use super::function::FunctionStmt;

#[derive(Clone, Debug)]
pub struct ClassStmt {
    pub name: Token,
    pub methods: Vec<Rc<FunctionStmt>>,
    pub superclass: Option<Expr>,
}

impl ClassStmt {
    pub fn superclass_name(&self) -> Option<String> {
        self.superclass
            .as_ref()
            .map(|expr| expr.to_variable().unwrap().name.lexeme.clone())
    }
}
