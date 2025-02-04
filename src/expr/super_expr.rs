use crate::token::Token;

#[derive(Clone, Debug)]
pub struct SuperExpr {
    pub keyword: Token,
    pub method: Token,
}

