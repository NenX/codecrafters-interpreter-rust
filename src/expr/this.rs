use crate::token::Token;

#[derive(Debug, Clone)]

pub struct ThisExpr {
    pub keyword: Token,
}