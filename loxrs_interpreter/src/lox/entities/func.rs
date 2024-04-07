use super::eval::Interpreter;
use super::stmt::StmtFun;
use super::Value;
use loxrs_env::Env;
use loxrs_types::Result;
use std::cell::RefCell;
use std::fmt::Result as fmt_result;
use std::fmt::{Debug, Formatter};

#[derive(Clone, PartialEq, Debug)]
pub enum Func {
    Lox(Function),
    Native(NativeFunction),
}

pub type FuncDefinition = fn(&mut Interpreter, RefCell<Env<Value>>) -> Result<Value>;

#[derive(Clone)]
pub struct Function {
    pub def: Box<StmtFun>,
    pub env: RefCell<Env<Value>>,
}

impl Function {
    pub fn arity(&self) -> usize {
        self.def.body.stmts.len()
    }

    pub fn name(&self) -> &str {
        self.def.name.extract_identifier_str().unwrap()
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        std::cmp::PartialEq::eq(&self.def, &other.def) && RefCell::eq(&self.env, &other.env)
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter) -> fmt_result {
        f.debug_struct("Function")
            .field("arity", &self.arity())
            .field("name", &self.name())
            .field("func", &"<function>")
            .field("env", &"<env>")
            .finish()
    }
}

#[derive(Clone)]
pub struct NativeFunction {
    pub def: FuncDefinition,
    pub env: RefCell<Env<Value>>,
    pub params: Vec<String>,
    pub name: String,
}

impl NativeFunction {
    pub fn new(
        def: FuncDefinition,
        env: RefCell<Env<Value>>,
        params: &[String],
        name: &str,
    ) -> Self {
        Self {
            env,
            name: name.to_owned(),
            def,
            params: params.to_owned().clone(),
        }
    }

    pub fn arity(&self) -> usize {
        self.params.len()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl PartialEq for NativeFunction {
    fn eq(&self, other: &Self) -> bool {
        std::cmp::PartialEq::eq(&self.def, &other.def)
            && RefCell::eq(&self.env, &other.env)
            && Vec::eq(&self.params, &other.params)
            && self.name == other.name
    }
}

impl Debug for NativeFunction {
    fn fmt(&self, f: &mut Formatter) -> fmt_result {
        f.debug_struct("NativeFunction")
            .field("arity", &self.arity())
            .field("name", &self.name())
            .field("func", &"<native fn>")
            .field("env", &"<env>")
            .finish()
    }
}
