use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Neg, Not, Sub},
};

#[derive(Clone, PartialEq, PartialOrd)]
pub enum Scalar {
    Bool(bool),
    Number(f64),
    String(String),
    Nil,
}
impl Scalar {
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
        match left {
            Scalar::Bool(_) => matches!(other, Scalar::Bool(_)),
            Scalar::Number(_) => matches!(other, Scalar::Number(_)),
            Scalar::String(_) => matches!(other, Scalar::String(_)),
            Scalar::Nil => matches!(other, Scalar::Nil),
        }
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
        };
        write!(f, "{}", s)
    }
}
