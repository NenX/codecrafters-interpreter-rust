use crate::expr::Expr;

use super::Stmt;

#[derive(Clone, Debug)]

pub struct WhileStmt {
    pub condition: Expr,
    pub body: Stmt,
}
