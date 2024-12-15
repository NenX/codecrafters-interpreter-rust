use binary::Binary;
use grouping::Grouping;
use literal::Literal;
use unary::Unary;

pub mod ast_printer;
pub mod ast_interpreter;
pub mod binary;
pub mod grouping;
pub mod literal;
pub mod unary;

#[derive(Clone, Debug)]
pub enum Expr {
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
    Literal(Box<Literal>),
    Unary(Box<Unary>),
}
impl From<Binary> for Expr {
    fn from(value: Binary) -> Self {
        Self::Binary(value.into())
    }
}
impl From<Grouping> for Expr {
    fn from(value: Grouping) -> Self {
        Self::Grouping(value.into())
    }
}
impl From<Literal> for Expr {
    fn from(value: Literal) -> Self {
        Self::Literal(value.into())
    }
}
impl From<Unary> for Expr {
    fn from(value: Unary) -> Self {
        Self::Unary(value.into())
    }
}
