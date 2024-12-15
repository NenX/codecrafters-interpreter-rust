use crate::data_types::scaler::Scalar;



#[derive(Clone, Debug)]
pub struct Literal {
    pub value: Scalar,
}
impl Literal {
    pub fn nil() -> Self {
        Self {
            value: Scalar::Nil,
        }
    }
}
impl From<bool> for Literal {
    fn from(value: bool) -> Self {
        Self {
            value: Scalar::Bool(value),
        }
    }
}
impl From<i64> for Literal {
    fn from(value: i64) -> Self {
        Self {
            value: Scalar::Number(value as f64),
        }
    }
}
impl From<f64> for Literal {
    fn from(value: f64) -> Self {
        Self {
            value: Scalar::Number(value),
        }
    }
}
impl From<&str> for Literal {
    fn from(value: &str) -> Self {
        Self {
            value: Scalar::String(value.to_string()),
        }
    }
}
