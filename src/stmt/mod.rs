use block::BlockStmt;
use expression::ExpressionStmt;
use if_stmt::IfStmt;
use print::PrintStmt;
use var::VarStmt;
use while_stmt::WhileStmt;

pub mod block;
pub mod expression;
pub mod if_stmt;
pub mod print;
pub mod var;
pub mod while_stmt;

#[derive(Clone, Debug)]
pub enum Stmt {
    Var(Box<VarStmt>),
    Expression(Box<ExpressionStmt>),
    Block(Box<BlockStmt>),
    Print(Box<PrintStmt>),
    If(Box<IfStmt>),
    While(Box<WhileStmt>),
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
impl From<IfStmt> for Stmt {
    fn from(value: IfStmt) -> Self {
        Self::If(value.into())
    }
}
impl From<WhileStmt> for Stmt {
    fn from(value: WhileStmt) -> Self {
        Self::While(value.into())
    }
}
