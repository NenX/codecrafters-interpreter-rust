use std::collections::HashMap;

use crate::{
    callable::Callable,
    data_types::scaler::Scalar,
    environment::{Environment, EnvironmentType},
    error::report_runtime,
    expr::Expr,
    stmt::Stmt,
    token::Token,
    InterpretRtErr,
};

use super::{error::InterpretResult, InterpretError, Interprete};

pub struct Evaluator {
    pub(crate) locals: HashMap<*const Expr, usize>,
    pub(crate) env: EnvironmentType,
    pub(crate) global: EnvironmentType,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator {
    pub fn new() -> Self {
        let global = Environment::global_env();
        Self {
            locals: HashMap::new(),
            env: global.clone(),
            global,
        }
    }
    pub fn resolve(&mut self, expr: &Expr, depth: usize) {
        self.locals.insert(expr, depth);
    }
    pub fn get_depth(&self, expr: &Expr) -> Option<usize> {
        let ptr = expr as *const Expr;
        let result = self.locals.get(&ptr).copied();
        result
    }
    pub fn eval_block(
        &mut self,
        statments: &Vec<Stmt>,
        new_env: EnvironmentType,
    ) -> InterpretResult<()> {
    
        let old_env = self.env.clone();
        self.env = new_env;
        // 使用闭包来捕获环境, 防止环境无法恢复
        let result = (|| {
            for stmt in statments {
                self.eval(stmt)?;
            }
            Ok(())
        })();

        self.env = old_env;
        result
    }
    pub(crate) fn check_number_operands(
        &self,
        left: &Scalar,
        right: &Scalar,
        operator: &Token,
    ) -> InterpretResult<()> {
        if Scalar::check_number_operands(left, right) {
            Ok(())
        } else {
            report_runtime(operator.line, "Operands must be numbers.".to_string());
            InterpretRtErr!(;"bad eval")
        }
    }

    pub(crate) fn check_number_operand(
        &self,
        right: &Scalar,
        operator: &Token,
    ) -> InterpretResult<()> {
        if matches!(right, Scalar::Number(_)) {
            Ok(())
        } else {
            report_runtime(operator.line, "Operand must be a number.".to_string());
            InterpretRtErr!(;"bad eval")
        }
    }
}
