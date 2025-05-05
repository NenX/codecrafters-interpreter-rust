use std::ptr::addr_of;

use assign::AssignExpr;
use binary::BinaryExpr;
use call::CallExpr;
use get::GetExpr;
use grouping::GroupingExpr;
use literal::LiteralExpr;
use logical::LogicalExpr;
use set::SetExpr;
use super_expr::SuperExpr;
use this::ThisExpr;
use unary::UnaryExpr;
use variable::VariableExpr;

pub mod assign;
pub mod binary;
pub mod call;
pub mod get;
pub mod grouping;
pub mod literal;
pub mod logical;
pub mod set;
pub mod super_expr;
pub mod this;
pub mod unary;
pub mod variable;

#[derive(Clone, Debug)]
pub enum Expr {
    Variable(Box<VariableExpr>),
    Assign(Box<AssignExpr>),
    Binary(Box<BinaryExpr>),
    Logical(Box<LogicalExpr>),
    Grouping(Box<GroupingExpr>),
    Literal(Box<LiteralExpr>),
    Call(Box<CallExpr>),
    Unary(Box<UnaryExpr>),
    Get(Box<GetExpr>),
    Set(Box<SetExpr>),
    This(Box<ThisExpr>),
    Super(Box<SuperExpr>),
}
impl Expr {
    pub fn to_variable(&self) -> Option<&VariableExpr> {
        match self {
            Self::Variable(variable) => Some(variable),
            _ => None,
        }
    }
    pub(crate) fn as_ptr(&self) -> usize {
        addr_of!(*self) as usize
    }
}
impl From<BinaryExpr> for Expr {
    fn from(value: BinaryExpr) -> Self {
        Self::Binary(value.into())
    }
}
impl From<GroupingExpr> for Expr {
    fn from(value: GroupingExpr) -> Self {
        Self::Grouping(value.into())
    }
}
impl From<LiteralExpr> for Expr {
    fn from(value: LiteralExpr) -> Self {
        Self::Literal(value.into())
    }
}
impl From<UnaryExpr> for Expr {
    fn from(value: UnaryExpr) -> Self {
        Self::Unary(value.into())
    }
}
impl From<VariableExpr> for Expr {
    fn from(value: VariableExpr) -> Self {
        Self::Variable(value.into())
    }
}
impl From<AssignExpr> for Expr {
    fn from(value: AssignExpr) -> Self {
        Self::Assign(value.into())
    }
}
impl From<LogicalExpr> for Expr {
    fn from(value: LogicalExpr) -> Self {
        Self::Logical(value.into())
    }
}
impl From<CallExpr> for Expr {
    fn from(value: CallExpr) -> Self {
        Self::Call(value.into())
    }
}
impl From<GetExpr> for Expr {
    fn from(value: GetExpr) -> Self {
        Self::Get(value.into())
    }
}
impl From<SetExpr> for Expr {
    fn from(value: SetExpr) -> Self {
        Self::Set(value.into())
    }
}
impl From<ThisExpr> for Expr {
    fn from(value: ThisExpr) -> Self {
        Self::This(value.into())
    }
}
impl From<SuperExpr> for Expr {
    fn from(value: SuperExpr) -> Self {
        Self::Super(value.into())
    }
}
impl ToString for Expr {
    fn to_string(&self) -> String {
        match self {
            Self::Variable(variable) => variable.name.lexeme.clone(),
            Self::This(_) => "this".to_string(),
            Self::Super(_) => "super".to_string(),
            _ => "expr".to_string(),
        }
    }
}
