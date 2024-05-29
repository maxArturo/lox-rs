use std::fmt::Display;

// runtime representation of a Lox Class.
#[derive(Clone, PartialEq, Debug)]
pub struct Class {
    pub name: String,
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[<Class {}>]", self.name)
    }
}
