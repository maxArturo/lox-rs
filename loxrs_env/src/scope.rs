#[cfg(test)]
mod test;

use core::fmt::{self, Display};
use log::trace;
use loxrs_types::{LoxErr, Result};
use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

type Link<T> = Option<Rc<Scope<T>>>;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Scope<T> {
    values: RefCell<HashMap<String, T>>,
    pub parent: Link<T>,
    pub globals: Link<T>,
}

impl<T: Display> Scope<T> {
    fn format_with_indent(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "Scope: {{")?;

        for (key, value) in self.values.borrow().iter() {
            for _ in 0..(indent + 1) {
                write!(f, "  ")?;
            }
            writeln!(f, "[{}: {}] ", key, value)?;
        }

        if let Some(p) = &self.parent {
            for _ in 0..(indent + 1) {
                write!(f, "  ")?;
            }
            p.as_ref().format_with_indent(f, indent + 1)?;
        }
        for _ in 0..(indent) {
            write!(f, "  ")?;
        }
        writeln!(f, "}}")?;
        Ok(())
    }
}

impl<T: Display + Clone> Scope<T> {
    pub fn new() -> Self {
        Self {
            values: RefCell::new(HashMap::default()),
            parent: None,
            globals: Self::new_globals(),
        }
    }

    fn new_globals() -> Link<T> {
        Some(Rc::new(Self {
            values: RefCell::new(HashMap::default()),
            parent: None,
            globals: None,
        }))
    }

    pub fn from_parent(parent: Rc<Scope<T>>) -> Rc<Scope<T>> {
        let globals = parent.as_ref().globals.clone();
        Rc::new(Self {
            values: RefCell::new(HashMap::new()),
            parent: Some(parent),
            globals,
        })
    }

    pub fn define(&self, name: &str, val: T) {
        self.values.borrow_mut().insert(name.to_string(), val);
    }

    pub fn define_global(&self, name: &str, val: T) {
        if let Some(el) = self.globals.as_ref() {
            el.define(name, val)
        }
    }

    pub fn assign(&self, name: &str, val: T) -> Result<()> {
        trace!("[assign] current env:\n{}", self);
        trace!(
            "[assign] adding to current env: name=[{}], value=[{}]",
            name,
            val
        );
        if self.values.borrow().contains_key(name) {
            self.values.borrow_mut().insert(name.to_string(), val);
            return Ok(());
        }
        match &self.parent {
            Some(parent) => parent.as_ref().assign(name, val),
            None => Err(LoxErr::Undefined {
                message: format!("attempted to assign to undefined var: {}", name),
            }),
        }
    }

    pub fn get(&self, name: &str) -> Result<T> {
        trace!("[get] looking for: `{}` in:\n{}", name, self);
        self.values
            .borrow()
            .get(name)
            .cloned()
            .or_else(|| {
                trace!("[get] going into parent...");
                self.parent
                    .as_ref()
                    .and_then(|parent| parent.as_ref().get(name).ok())
            })
            .or_else(|| {
                trace!("[get] going into globals...");
                self.globals
                    .as_ref()
                    .and_then(|globals| globals.get(name).ok())
            })
            .ok_or(LoxErr::Undefined {
                message: format!("variable undefined: {}", name),
            })
    }
}

impl<T: Display> Display for Scope<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.format_with_indent(f, 0)
    }
}
