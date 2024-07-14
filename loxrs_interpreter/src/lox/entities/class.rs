use std::{fmt::Display, rc::Rc};

use super::func::Function;

#[derive(Clone, PartialEq, Debug)]
pub struct Class {
    pub name: String,
    pub methods: Vec<Function>,
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[<Class {}>]", self.name)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Instance {
    class: Rc<Class>,
}

impl Instance {
    pub fn new(class: Rc<Class>) -> Self {
        Self { class }
    }
}

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[<Instance {}>]", self.class.name)
    }
}
