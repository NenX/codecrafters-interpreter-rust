use block::BlockStmt;
use expression::ExpressionStmt;
use print::PrintStmt;
use var::VarStmt;

pub mod block;
pub mod expression;
pub mod print;
pub mod var;

#[derive(Clone, Debug)]
pub enum Stmt {
    Var(Box<VarStmt>),
    Expression(Box<ExpressionStmt>),
    Block(Box<BlockStmt>),
    Print(Box<PrintStmt>),
}
impl From<VarStmt> for Stmt {
    fn from(value: VarStmt) -> Self {
        Self::Var(value.into())
    }
}
impl From<ExpressionStmt> for Stmt {
    fn from(value: ExpressionStmt) -> Self {
        Self::Expression(value.into())
    }
}
impl From<BlockStmt> for Stmt {
    fn from(value: BlockStmt) -> Self {
        Self::Block(value.into())
    }
}
impl From<PrintStmt> for Stmt {
    fn from(value: PrintStmt) -> Self {
        Self::Print(value.into())
    }
}
