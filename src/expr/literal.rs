use crate::token::Token;

use super::Expr;

#[derive(Clone, Debug)]
pub enum LiteralValue {
    Bool(bool),
    Number(f64),
    String(String),
    Nil,
}
#[derive(Clone, Debug)]
pub struct Literal {
    pub value: LiteralValue,
}
impl Literal {
    pub fn nil() -> Self {
        Self {
            value: LiteralValue::Nil,
        }
    }
}
impl From<bool> for Literal {
    fn from(value: bool) -> Self {
        Self {
            value: LiteralValue::Bool(value),
        }
    }
}
impl From<i64> for Literal {
    fn from(value: i64) -> Self {
        Self {
            value: LiteralValue::Number(value as f64),
        }
    }
}
impl From<f64> for Literal {
    fn from(value: f64) -> Self {
        Self {
            value: LiteralValue::Number(value),
        }
    }
}
impl From<&str> for Literal {
    fn from(value: &str) -> Self {
        Self {
            value: LiteralValue::String(value.to_string()),
        }
    }
}
