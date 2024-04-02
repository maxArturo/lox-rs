use log::debug;
use loxrs_types::{LoxErr, Result};
use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

type Link<T> = Option<Box<Scope<T>>>;
type Globals<T> = Option<Rc<Scope<T>>>;

#[derive(Debug, Clone, PartialEq)]
struct Scope<T> {
    values: RefCell<HashMap<String, T>>,
    pub parent: Link<T>,
    pub globals: Globals<T>,
}

impl<T> Scope<T>
where
    T: Debug + Clone,
{
    fn new_globals() -> Globals<T> {
        Some(Rc::new(Self {
            values: RefCell::new(HashMap::default()),
            parent: None,
            globals: None,
        }))
    }

    fn with_parent(parent: Link<T>, globals: Globals<T>) -> Link<T> {
        Some(Box::new(Self {
            values: RefCell::new(HashMap::new()),
            parent,
            globals: globals.map(|e| Rc::clone(&e)),
        }))
    }

    fn define(&self, name: &str, val: T) {
        self.values.borrow_mut().insert(name.to_string(), val);
    }

    fn assign(&self, name: &str, val: T) -> Result<()> {
        debug!("[assign] curr values: {:?}", self);
        if self.values.borrow().contains_key(name) {
            self.values.borrow_mut().insert(name.to_string(), val);
            return Ok(());
        }
        match &self.parent {
            Some(parent) => parent.assign(name, val),
            None => Err(LoxErr::Undefined {
                message: format!("attempted to assign to undefined var: {}", name),
            }),
        }
    }

    fn get(&self, name: &str) -> Result<T> {
        debug!("[get] curr values: {:?}", self);
        self.values
            .borrow()
            .get(name)
            .cloned()
            .or_else(|| {
                self.parent
                    .as_ref()
                    .and_then(|parent| parent.get(name).ok())
            })
            .or_else(|| {
                self.globals
                    .as_ref()
                    .and_then(|parent| parent.get(name).ok())
            })
            .ok_or(LoxErr::Undefined {
                message: format!("variable undefined: {}", name),
            })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Env<T> {
    current: Link<T>,
    globals: Globals<T>,
}

impl<T> Env<T>
where
    T: Debug + Clone,
{
    pub fn new() -> Self {
        let globals = Scope::new_globals();
        let current = Scope::with_parent(None, globals.as_ref().map(Rc::clone));
        Self { globals, current }
    }

    pub fn define_global(&mut self, name: &str, val: T) {
        if let Some(s) = self.globals.as_ref() {
            s.define(name, val);
        }
    }

    pub fn open_scope(&mut self) {
        self.current =
            Scope::with_parent(self.current.take(), self.globals.as_ref().map(Rc::clone));
    }

    pub fn close_scope(&mut self) {
        if let Some(mut s) = self.current.take() {
            self.current = s.parent.take();
        }
    }

    pub fn define(&mut self, name: &str, val: T) {
        if let Some(s) = self.current.as_ref() {
            s.define(name, val);
        }
    }

    pub fn assign(&mut self, name: &str, val: T) -> Result<()> {
        self.current
            .as_ref()
            .ok_or(LoxErr::Internal {
                message: "Attempted to assign to an unscoped environment".to_string(),
            })
            .and_then(|s| s.assign(name, val))
    }

    pub fn get(&self, name: &str) -> Result<T> {
        match &self.current {
            Some(current) => current.get(name),
            None => Err(LoxErr::Internal {
                message: "Attempted to get from an unscoped environment".to_string(),
            }),
        }
    }
}

impl<T> Default for Env<T>
where
    T: Debug + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for Env<T> {
    fn drop(&mut self) {
        let mut curr = self.current.take();
        while let Some(mut link) = curr {
            curr = link.parent.take();
        }
        self.globals.take();
    }
}
