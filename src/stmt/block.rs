
use super::Stmt;

#[derive(Clone, Debug)]

pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}
