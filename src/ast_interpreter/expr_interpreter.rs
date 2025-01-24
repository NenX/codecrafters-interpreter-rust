use crate::{
    callable::Callable,
    data_types::scaler::Scalar,
    environment::{Environment, EnvironmentType},
    error::report_runtime,
    evaluator::Evaluator,
    expr::{binary::BinaryExpr, Expr},
    token::Token,
    token_type::TokenType,
    InterpretRtErr,
};

use super::{
    interpret_err::{InterpretError, InterpretResult},
    AstInterpreter,
};

