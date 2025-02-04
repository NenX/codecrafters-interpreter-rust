use std::collections::HashMap;

use crate::{
    error::my_error_token,
    evaluator::Evaluator,
    expr::Expr,
    stmt::{function::FunctionStmt, Stmt},
    token::Token,
};

use super::ResolverWalk;
#[derive(Clone, Copy, Debug)]
pub enum FunctionType {
    None,
    Function,
    Initializer,
    Method,
}
#[derive(Clone, Copy, Debug)]
pub enum ClassType {
    None,
    Class,
    Subclass,
}
pub struct Resolver<'a> {
    scopes: Vec<HashMap<String, bool>>,
    evaluator: &'a mut Evaluator,
    pub function_type: FunctionType,
    pub class_type: ClassType,
}

impl<'a> Resolver<'a> {
    pub fn new(evaluator: &'a mut Evaluator) -> Self {
        Self {
            scopes: vec![],
            evaluator,
            function_type: FunctionType::None,
            class_type: ClassType::None,
        }
    }
    pub fn set_function_type(&mut self, function_type: FunctionType) {
        self.function_type = function_type;
    }
    pub fn set_class_type(&mut self, class_type: ClassType) {
        self.class_type = class_type;
    }

    pub fn is_function(&self) -> bool {
        !matches!(self.function_type, FunctionType::None)
    }
    pub fn is_initializer(&self) -> bool {
        matches!(self.function_type, FunctionType::Initializer)
    }
    pub fn is_method(&self) -> bool {
        matches!(self.function_type, FunctionType::Method)
    }
    pub fn is_normal_function(&self) -> bool {
        matches!(self.function_type, FunctionType::Function)
    }
    pub fn is_class_none(&self) -> bool {
        matches!(self.class_type, ClassType::None)
    }
    pub fn is_class(&self) -> bool {
        matches!(self.class_type, ClassType::Class)
    }
    pub fn is_subclass(&self) -> bool {
        matches!(self.class_type, ClassType::Subclass)
    }
    pub fn resolve_function(&mut self, function: &FunctionStmt, function_type: FunctionType) {
        let enclosing_function = self.function_type;
        self.set_function_type(function_type);

        // self.declare(&function.name);
        // self.define(&function.name);
        self.begin_scope();
        for param in function.params.iter() {
            self.declare(param);
            self.define(param);
        }
        self.resolve_stmts(&function.fn_body);
        self.end_scope();

        self.set_function_type(enclosing_function);
    }
    pub fn resolve_stmts(&mut self, stmts: &Vec<Stmt>) {
        for stmt in stmts {
            self.resolve(stmt);
        }
    }
    pub fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    pub fn end_scope(&mut self) {
        self.scopes.pop();
    }
    pub fn declare(&mut self, token: &Token) {
        let name = token.lexeme.clone();
        let len = self.scopes.len();
        if len == 0 {
            return;
        }
        let scope = &mut self.scopes[len - 1];
        if scope.contains_key(&name) {
            my_error_token(
                token.clone(),
                "Already a variable with this name in this scope.".to_string(),
            );
        }
        scope.insert(name, false);
    }
    pub fn define(&mut self, token: &Token) {
        let name = token.lexeme.clone();
        let len = self.scopes.len();
        if len == 0 {
            return;
        }
        // 不需要检查是否已经存在，因为定义时，变量的声明不一定在当前作用域
        self.scopes[len - 1].insert(name, true);
    }
    pub fn resolve_local(&mut self, expr: &Expr, name: &str) {
        for (i, item) in self.scopes.iter().rev().enumerate() {
            if item.contains_key(name) {
                self.evaluator.resolve(expr, i);
                return;
            }
        }
    }
    pub fn cur_scope(&mut self) -> Option<&mut HashMap<String, bool>> {
        let len = self.scopes.len();
        if len == 0 {
            return None;
        }
        Some(&mut self.scopes[len - 1])
    }
}
