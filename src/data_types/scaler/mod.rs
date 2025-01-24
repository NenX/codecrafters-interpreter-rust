use std::{
    fmt::{format, Debug, Display},
    ops::{Add, Div, Mul, Neg, Not, Sub},
};
mod function_value;

pub use function_value::*;

use crate::callable::Callable;

// #[derive(Clone, PartialEq, PartialOrd)]
pub enum Scalar {
    Bool(bool),
    Number(f64),
    String(String),
    Function(FunctionValue),
    Nil,
}
impl Clone for Scalar {
    fn clone(&self) -> Self {
        match self {
            Scalar::Bool(x) => Scalar::Bool(x.clone()),
            Scalar::Number(x) => Scalar::Number(x.clone()),
            Scalar::String(x) => Scalar::String(x.clone()),
            Scalar::Function(callable) => Scalar::Function(callable.clone()),
            Scalar::Nil => Scalar::Nil,
        }
    }
}

impl PartialEq for Scalar {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
impl PartialOrd for Scalar {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Bool(l0), Self::Bool(r0)) => l0.partial_cmp(r0),
            (Self::Number(l0), Self::Number(r0)) => l0.partial_cmp(r0),
            (Self::String(l0), Self::String(r0)) => l0.partial_cmp(r0),
            _ => None,
        }
    }
}
impl Scalar {
    pub(crate) fn as_fun(&self) -> Option<FunctionValue> {
        match self {
            Scalar::Function(f) => Some(f.clone()),
            _ => None,
        }
    }
    pub(crate) fn as_bool(&self) -> Option<bool> {
        match self {
            Scalar::Bool(f) => Some(*f),
            _ => None,
        }
    }
    pub(crate) fn as_number(&self) -> Option<f64> {
        match self {
            Scalar::Number(f) => Some(*f),
            _ => None,
        }
    }
    pub(crate) fn as_string(&self) -> Option<String> {
        match self {
            Scalar::String(f) => Some(f.clone()),
            _ => None,
        }
    }
    pub(crate) fn is_same_type(left: &Self, other: &Self) -> bool {
        std::mem::discriminant(left) == std::mem::discriminant(other)
    }
    pub(crate) fn check_number_operands(left: &Self, right: &Self) -> bool {
        matches!((left, right), (Scalar::Number(_), Scalar::Number(_)))
    }
    pub(crate) fn check_string_operands(left: &Self, right: &Self) -> bool {
        matches!((left, right), (Scalar::String(_), Scalar::String(_)))
    }
}
impl Add for Scalar {
    type Output = Scalar;

    fn add(self, rhs: Self) -> Self::Output {
        if Self::check_number_operands(&self, &rhs) {
            let f1 = self.as_number().unwrap();
            let f2 = rhs.as_number().unwrap();
            return Scalar::Number(f1 + f2);
        }
        if Self::check_string_operands(&self, &rhs) {
            let f1 = self.as_string().unwrap();
            let f2 = rhs.as_string().unwrap();
            let s = f1 + &f2;
            return Scalar::String(s);
        }
        Scalar::Nil
    }
}
impl Sub for Scalar {
    type Output = Scalar;
    fn sub(self, rhs: Self) -> Self::Output {
        if Self::check_number_operands(&self, &rhs) {
            let f1 = self.as_number().unwrap();
            let f2 = rhs.as_number().unwrap();
            return Scalar::Number(f1 - f2);
        }
        Scalar::Nil
    }
}
impl Mul for Scalar {
    type Output = Scalar;
    fn mul(self, rhs: Self) -> Self::Output {
        if Self::check_number_operands(&self, &rhs) {
            let f1 = self.as_number().unwrap();
            let f2 = rhs.as_number().unwrap();
            return Scalar::Number(f1 * f2);
        }
        Scalar::Nil
    }
}
impl Div for Scalar {
    type Output = Scalar;
    fn div(self, rhs: Self) -> Self::Output {
        if Self::check_number_operands(&self, &rhs) {
            let f1 = self.as_number().unwrap();
            let f2 = rhs.as_number().unwrap();
            return Scalar::Number(f1 / f2);
        }
        Scalar::Nil
    }
}
impl Neg for Scalar {
    type Output = Scalar;
    fn neg(self) -> Self::Output {
        let f1 = self.as_number();
        match f1 {
            Some(f) => Scalar::Number(-f),
            None => Scalar::Nil,
        }
    }
}
impl Not for Scalar {
    type Output = Scalar;
    fn not(self) -> Self::Output {
        match self {
            Scalar::Bool(b) => Scalar::Bool(!b),
            Scalar::Number(_) => Scalar::Bool(false),
            Scalar::String(_) => Scalar::Bool(false),
            Scalar::Nil => Scalar::Bool(true),
            Scalar::Function(_) => Scalar::Bool(false),
        }
    }
}
// impl std::cmp::PartialEq for Scalar {
//     fn eq(&self, other: &Self) -> bool {
//         todo!()
//     }
// }
// impl std::cmp::PartialOrd for Scalar {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         todo!()
//     }
// }
impl Display for Scalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Scalar::Bool(b) => format!("{:?}", b),
            Scalar::Number(i) => format!("{}", i),
            Scalar::String(s) => s.clone(),
            Scalar::Nil => format!("nil"),
            Scalar::Function(function_value) => format!("{}", function_value.to_string()),
        };
        write!(f, "{}", s)
    }
}
impl Debug for Scalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Scalar::Bool(b) => format!("{:?}", b),
            Scalar::Number(i) => format!("{:?}", i),
            Scalar::String(s) => s.clone(),
            Scalar::Nil => format!("nil"),
            Scalar::Function(function_value) => format!("fn {}", function_value.to_string()),
        };
        write!(f, "{}", s)
    }
}
impl From<String> for Scalar {
    fn from(value: String) -> Self {
        Self::String(value.into())
    }
}
impl From<&str> for Scalar {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}
impl From<NativeFn> for Scalar {
    fn from(value: NativeFn) -> Self {
        Self::Function(value.into())
    }
}
impl From<UserFn> for Scalar {
    fn from(value: UserFn) -> Self {
        Self::Function(value.into())
    }
}
