use crate::expr::Expr;

use super::Stmt;

#[derive(Clone, Debug)]

pub struct IfStmt {
    pub then_branch: Stmt,
    pub else_branch: Option<Stmt>,
    pub condition: Expr,
}
