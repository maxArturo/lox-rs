use super::eval::Interpreter;
use super::stmt::StmtFun;
use super::{Token, Value};
use loxrs_env::Scope;
use loxrs_types::Result;
use std::fmt::Result as fmt_result;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

#[derive(Clone, PartialEq, Debug)]
pub enum Func {
    Lox(Function),
    Native(NativeFunction),
}

pub type FuncDefinition = fn(&mut Interpreter, Rc<Scope<Value>>) -> Result<Value>;

#[derive(Clone)]
pub struct Function {
    pub def: Box<StmtFun>,
    pub scope: Rc<Scope<Value>>,
    pub params: Vec<Token>,
}

impl Function {
    pub fn arity(&self) -> usize {
        self.params.len()
    }

    pub fn name(&self) -> &str {
        self.def.name.extract_identifier_str().unwrap()
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        std::cmp::PartialEq::eq(&self.def, &other.def) && Rc::eq(&self.scope, &other.scope)
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
    pub scope: Rc<Scope<Value>>,
    pub params: Vec<String>,
    pub name: String,
}

impl NativeFunction {
    pub fn new(
        def: FuncDefinition,
        scope: Rc<Scope<Value>>,
        params: &[String],
        name: &str,
    ) -> Self {
        Self {
            scope,
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
            && Rc::eq(&self.scope, &other.scope)
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
