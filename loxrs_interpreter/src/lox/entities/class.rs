use loxrs_types::{LoxErr, Result as LoxRes};
use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use super::{
    func::{Func, Function},
    Value,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Class {
    pub name: String,
    pub superclass: Option<Rc<Class>>,
    pub methods: HashMap<String, Function>,
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[<Class {}>]", self.name)
    }
}

impl Class {
    pub fn find_method(&self, name: &str) -> Option<&Function> {
        self.methods
            .get(name)
            .or_else(|| self.superclass.as_ref().and_then(|s| s.find_method(name)))
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Instance {
    class: Rc<Class>,
    fields: Rc<RefCell<HashMap<String, Value>>>,
}

pub type InstanceRef = Rc<RefCell<Instance>>;

impl Instance {
    pub fn new(class: Rc<Class>) -> InstanceRef {
        Rc::new(RefCell::new(Self {
            class,
            fields: Rc::new(RefCell::new(HashMap::new())),
        }))
    }

    pub fn get(instance: InstanceRef, key: &str) -> LoxRes<Value> {
        let binding = instance.as_ref().borrow();
        let fields = binding.fields.as_ref().borrow();

        if fields.contains_key(key) {
            return Ok(fields.get(key).unwrap_or(&Value::Nil).clone());
        }

        match instance.as_ref().borrow().class.find_method(key) {
            Some(method) => Ok(Value::Func(Func::Lox(method.bind(Rc::clone(&instance))))),
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
