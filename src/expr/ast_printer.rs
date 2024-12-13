use std::fmt::format;

use crate::expr::binary::Binary;

use super::{literal::LiteralValue, Expr};

pub trait AstPrinter {
    fn print(&self) -> String;
    fn parenthesize(&self, name: &str, exprs: Vec<&Expr>) -> String;
}
impl AstPrinter for Expr {
    fn print(&self) -> String {
        match self {
            Expr::Binary(binary) => {
                let Binary {
                    letf,
                    right,
                    operator: opertor,
                } = binary.as_ref();
                self.parenthesize(&opertor.lexeme, [letf, right].to_vec())
            }
            Expr::Grouping(grouping) => self.parenthesize("group", [&grouping.expression].to_vec()),
            Expr::Literal(literal) => match &literal.value {
                LiteralValue::Bool(b) => format!("{:?}",b),
                LiteralValue::Number(i) => format!("{:?}",i),
                LiteralValue::String(s) => s.clone(),
                LiteralValue::Nil => format!("Nil"),
            },
            Expr::Unary(unary) => self.parenthesize(&unary.operator.lexeme, [&unary.right].to_vec()),
        }
    }

    fn parenthesize(&self, name: &str, exprs: Vec<&Expr>) -> String {
        let mut s = format!("({}", name);
        for exp in exprs {
            let _s = format!(" {}",exp.print());
            s.push_str(&_s);
        }
        s.push_str(")");
        s
    }
}
