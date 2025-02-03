use crate::expr::{binary::BinaryExpr, Expr};

pub trait AstPrinter {
    fn print(&self, debug: bool) -> String;
    fn parenthesize(&self, name: &str, exprs: Vec<&Expr>, debug: bool) -> String;
}
impl AstPrinter for Expr {
    fn print(&self, debug: bool) -> String {
        match self {
            Expr::Binary(binary) => {
                let BinaryExpr {
                    left: letf,
                    right,
                    operator: opertor,
                } = binary.as_ref();
                self.parenthesize(&opertor.lexeme, [letf, right].to_vec(),debug)
            }
            Expr::Grouping(grouping) => self.parenthesize("group", [&grouping.expression].to_vec(),debug),
            Expr::Literal(literal) => {
                if debug {
                    format!("{:?}", literal.value)
                } else {
                    format!("{}", literal.value)
                }
            }
            Expr::Unary(unary) => {
                self.parenthesize(&unary.operator.lexeme, [&unary.right].to_vec(),debug)
            }
            Expr::Variable(variable) => todo!(),
            Expr::Assign(assign) => todo!(),
            Expr::Logical(logical_expr) => todo!(),
            Expr::Call(call_expr) => todo!(),
            Expr::Get(get_expr) => todo!(),
            Expr::Set(set_expr) => todo!(),
            Expr::This(this_expr) => todo!(),
        }
    }

    fn parenthesize(&self, name: &str, exprs: Vec<&Expr>, debug: bool) -> String {
        let mut s = format!("({}", name);
        for exp in exprs {
            let _s = format!(" {}", exp.print(debug));
            s.push_str(&_s);
        }
        s.push(')');
        s
    }
}
