use crate::lox::interpreter::error::{LoxErr, Result};
use std::{collections::HashMap, fmt::Debug};

#[derive(Debug)]
pub struct Env<T> {
    values: HashMap<String, T>,
}

impl<T: Debug> Env<T> {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, val: T) {
        println!("inserting: {:?}", name);
        self.values.insert(name.to_string(), val);
        println!("env after insert: {:?}", self.values);
    }

    pub fn get(&self, name: &str) -> Result<&T> {
        println!("curr values: {:?}", self.values);
        self.values.get(name).ok_or(LoxErr::Undefined {
            message: format!("variable undefined: {}", name),
        })
    }
}
