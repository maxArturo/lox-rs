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
        }
    }

    pub fn from_parent(parent: Rc<Scope<T>>) -> Rc<Scope<T>> {
        Rc::new(Self {
            values: RefCell::new(HashMap::new()),
            parent: Some(parent),
        })
    }

    pub fn define(&self, name: &str, val: T) {
        self.values.borrow_mut().insert(name.to_string(), val);
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

    pub fn assign_at(&self, distance: usize, name: &str, val: T) -> Result<()> {
        self.ancestor(distance)?
            .values
            .borrow_mut()
            .insert(name.to_owned(), val);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Result<T> {
        trace!("[get] looking for: `{}` in:\n{}", name, self);
        self.values
            .borrow()
            .get(name)
            .cloned()
            .ok_or(LoxErr::Undefined {
                message: format!("variable undefined: {}", name),
            })
    }

    pub fn get_at(&self, distance: usize, name: &str) -> Result<T> {
        self.ancestor(distance)?.get(name)
    }

    fn ancestor(&self, distance: usize) -> Result<&Scope<T>> {
        let mut parent = self;

        for _i in 0..distance {
            parent = parent
            .parent.as_ref()
            .ok_or::<LoxErr>(LoxErr::Internal {
                message: "Error with ancestor depth! Expected to find some ancestor scope when resolving variable".to_owned(),
            })
            .map(|el| el.as_ref())?;
        }

        Ok(parent)
    }
}

impl<T: Display> Display for Scope<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.format_with_indent(f, 0)
    }
}
