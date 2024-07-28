use loxrs_types::{LoxErr, Result as LoxRes};
use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use super::{
    func::{Func, Function},
    Value,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Class {
    pub name: String,
    pub methods: HashMap<String, Function>,
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[<Class {}>]", self.name)
    }
}

impl Class {
    pub fn find_method(&self, name: &str) -> Option<&Function> {
        self.methods.get(name)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Instance {
    class: Rc<Class>,
    fields: Rc<RefCell<HashMap<String, Value>>>,
}

impl Instance {
    pub fn new(class: Rc<Class>) -> Self {
        Self {
            class,
            fields: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn get(&self, key: &str) -> LoxRes<Value> {
        let fields = self.fields.borrow();
        if fields.contains_key(key) {
            return Ok(fields.get(key).unwrap_or(&Value::Nil).clone());
        }

        match self.class.find_method(key) {
            // TODO add a "bind" method here to assign a "this" for method functions
            Some(method) => Ok(Value::Func(Func::Lox(method.clone()))),
            None => Err(LoxErr::Undefined {
                message: format!("undefined property: {}", key),
            }),
        }
    }

    /// [design_note]
    /// implicit shadowing of class methods by instance variables
    /// happens here
    pub fn set(&self, key: &str, val: Value) {
        self.fields.borrow_mut().insert(key.to_owned(), val);
    }
}

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[<Instance {}>, fields={:?}]",
            self.class.name, self.fields
        )
    }
}
