use crate::lox::interpreter::error::{LoxErr, Result};
use log::debug;
use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, Clone)]
pub struct Env<T> {
    current: Link<T>,
}

type Link<T> = Option<Box<Scope<T>>>;

#[derive(Debug, Clone)]
struct Scope<T> {
    values: HashMap<String, T>,
    pub parent: Link<T>,
}

impl<T> Env<T>
where
    T: Debug + Clone,
{
    pub fn new() -> Self {
        Self {
            current: Some(Box::new(Scope::new())),
        }
    }

    pub fn open_scope(&mut self) {
        let new_scope = Box::new(Scope::with_parent(self.current.take()));
        self.current = Some(new_scope);
    }

    pub fn close_scope(&mut self) {
        if let Some(s) = self.current.take() {
            self.current = s.parent;
        }
    }

    pub fn define(&mut self, name: &str, val: T) {
        if let Some(s) = self.current.as_mut() {
            s.define(name, val)
        }
    }

    pub fn assign(&mut self, name: &str, val: T) -> Result<()> {
        self.current
            .as_mut()
            .ok_or(LoxErr::Internal {
                message: "Attempted to assign to an unscoped environment".to_string(),
            })
            .and_then(|s| s.assign(name, val))?;
        Ok(())
    }

    pub fn get(&self, name: &str) -> Result<&T> {
        self.current
            .as_ref()
            .ok_or(LoxErr::Internal {
                message: "Attempted to get from an unscoped environment".to_string(),
            })
            .and_then(|s| s.get(name))
    }
}

impl<T> Drop for Env<T> {
    fn drop(&mut self) {
        let mut curr = self.current.take();
        while let Some(mut link) = curr {
            curr = link.parent.take();
        }
    }
}

impl<T> Scope<T>
where
    T: Debug + Clone,
{
    fn new() -> Self {
        Self {
            values: HashMap::new(),
            parent: None,
        }
    }

    fn with_parent(parent: Link<T>) -> Self {
        Self {
            values: HashMap::new(),
            parent,
        }
    }

    fn define(&mut self, name: &str, val: T) {
        self.values.insert(name.to_string(), val);
    }

    fn assign(&mut self, name: &str, val: T) -> Result<()> {
        debug!("[assign] curr values: {:?}", self);
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), val);
            return Ok(());
        }
        if self.parent.is_some() {
            self.parent.as_mut().unwrap().assign(name, val)?;
            return Ok(());
        }
        Err(LoxErr::Undefined {
            message: format!("attempted to assign to undefined var: {}", name),
        })
    }

    fn get(&self, name: &str) -> Result<&T> {
        debug!("[get] curr values: {:?}", self);
        self.values
            .get(name)
            .or_else(|| self.parent.as_ref().and_then(|env| env.get(name).ok()))
            .ok_or(LoxErr::Undefined {
                message: format!("variable undefined: {}", name),
            })
    }
}
