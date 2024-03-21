use crate::lox::interpreter::error::{LoxErr, Result};
use log::debug;
use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, Clone)]
pub struct Env<T> {
    values: HashMap<String, T>,
    pub enclosing: Option<Box<Env<T>>>,
}

impl<T> Env<T>
where
    T: Debug + Clone,
{
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn with_env(env: Env<T>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(Box::new(env)),
        }
    }

    pub fn define(&mut self, name: &str, val: T) {
        self.values.insert(name.to_string(), val);
    }

    pub fn assign(&mut self, name: &str, val: T) -> Result<()> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), val);
            return Ok(());
        }
        if self
            .enclosing
            .as_mut()
            .and_then(|e| e.values.contains_key(name).then_some(true))
            .is_some()
        {
            self.enclosing.as_mut().unwrap().assign(name, val)?;
            return Ok(());
        }
        Err(LoxErr::Undefined {
            message: format!("attempted to assign to undefined var: {}", name),
        })
    }

    pub fn get(&self, name: &str) -> Result<&T> {
        debug!("[get] curr values: {:?}", self);
        self.values
            .get(name)
            .or_else(|| self.enclosing.as_ref().and_then(|env| env.get(name).ok()))
            .ok_or(LoxErr::Undefined {
                message: format!("variable undefined: {}", name),
            })
    }
}
