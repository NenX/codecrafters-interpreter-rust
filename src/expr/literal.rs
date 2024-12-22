use crate::data_types::scaler::Scalar;



#[derive(Clone, Debug)]
pub struct LiteralExpr {
    pub value: Scalar,
}
impl LiteralExpr {
    pub fn nil() -> Self {
        Self {
            value: Scalar::Nil,
        }
    }
}
impl From<bool> for LiteralExpr {
    fn from(value: bool) -> Self {
        Self {
            value: Scalar::Bool(value),
        }
    }
}
impl From<i64> for LiteralExpr {
    fn from(value: i64) -> Self {
        Self {
            value: Scalar::Number(value as f64),
        }
    }
}
impl From<f64> for LiteralExpr {
    fn from(value: f64) -> Self {
        Self {
            value: Scalar::Number(value),
        }
    }
}
impl From<&str> for LiteralExpr {
    fn from(value: &str) -> Self {
        Self {
            value: Scalar::String(value.to_string()),
        }
    }
}
