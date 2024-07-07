use std::fmt::Display;

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
