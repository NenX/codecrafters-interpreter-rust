use std::collections::HashMap;

use crate::{
    data_types::scaler::Scalar,
    environment::{EnvErr, Environment, EnvironmentType},
    error::report_runtime,
    expr::Expr,
    stmt::Stmt,
    token::Token,
    InterpretRtErr,
};

use super::{error::InterpretResult, InterpretError, Interprete};

pub struct Evaluator {
    pub(crate) locals: HashMap<usize, (usize, String)>,
    pub(crate) env: EnvironmentType,
    pub(crate) global: EnvironmentType,
    pub(crate) resolver: bool,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new(false)
    }
}

impl Evaluator {
    pub fn new(resolver: bool) -> Self {
        let global = Environment::global_env();
        Self {
            locals: HashMap::new(),
            env: global.clone(),
            global,
            resolver,
        }
    }
    pub(crate) fn resolve(&mut self, expr: &Expr, depth: usize) {
        self.locals.insert(expr.as_ptr(), (depth, expr.to_string()));
    }
    pub(crate) fn get_depth(&self, expr: &Expr) -> Option<usize> {
        let ptr = expr.as_ptr();
        // println!("ptr: {:?} name: {} local: {:?}", ptr, expr.to_string(), self.locals);

        let result = self.locals.get(&ptr).cloned();
        result.map(|(depth, _)| depth)
    }
    pub(crate) fn assign_variable(
        &mut self,
        expr: &Expr,
        name: &str,
        value: Scalar,
    ) -> Result<(), EnvErr> {
        if !self.resolver {
            return self.env.borrow_mut().assign(name, value);
        }
        let distance = self.get_depth(expr);
        if let Some(distance) = distance {
            self.env.borrow_mut().assign_at(distance, name, value)
        } else {
            self.global.borrow_mut().assign(name, value)
        }
    }
    pub(crate) fn lookup_variable(&mut self, expr: &Expr, name: &str) -> Result<Scalar, EnvErr> {
        if !self.resolver {
            return self.env.borrow().get(name);
        }
        let distance = self.get_depth(expr);
        if let Some(distance) = distance {
            self.env.borrow().get_at(distance, name)
        } else {
            self.global.borrow().get(name)
        }
    }
    pub(crate) fn eval_block(
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
