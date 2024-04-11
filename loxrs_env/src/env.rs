use core::fmt::{self, Display};
use log::trace;
use loxrs_types::{LoxErr, Result};
use std::borrow::Borrow;
use std::fmt::Result as fmt_result;
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{Debug, Formatter},
    rc::Rc,
};

type Link<T> = Option<Rc<RefCell<Scope<T>>>>;
type Globals<T> = Option<Rc<Scope<T>>>;

#[derive(Clone, Debug, PartialEq)]
struct Scope<T> {
    values: RefCell<HashMap<String, T>>,
    pub parent: Link<T>,
    pub globals: Globals<T>,
}

impl<T: Display> Scope<T> {
    fn format_with_indent(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        for _ in 0..(indent) {
            write!(f, "  ")?;
        }
        writeln!(f, "Scope: {{")?;

        for _ in 0..(indent) {
            write!(f, "  ")?;
        }
        for (key, value) in self.values.borrow().iter() {
            writeln!(f, "[{}: {}] ", key, value)?;
        }

        if let Some(p) = &self.parent {
            writeln!(f)?;
            for _ in 0..(indent + 1) {
                write!(f, "  ")?;
            }
            p.as_ref().borrow().format_with_indent(f, indent + 1)?;
        }

        writeln!(f, "}}")?;
        Ok(())
    }
}

impl<T: Display + Clone> Scope<T> {
    fn new_globals() -> Globals<T> {
        Some(Rc::new(Self {
            values: RefCell::new(HashMap::default()),
            parent: None,
            globals: None,
        }))
    }

    fn with_parent(parent: Link<T>, globals: Globals<T>) -> Link<T> {
        Some(Rc::new(RefCell::new(Self {
            values: RefCell::new(HashMap::new()),
            parent,
            globals: globals.map(|e| Rc::clone(&e)),
        })))
    }

    fn define(&self, name: &str, val: T) {
        self.values.borrow_mut().insert(name.to_string(), val);
    }

    fn assign(&self, name: &str, val: T) -> Result<()> {
        trace!("[assign] current env: {}", self);
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
            Some(parent) => parent.as_ref().borrow_mut().assign(name, val),
            None => Err(LoxErr::Undefined {
                message: format!("attempted to assign to undefined var: {}", name),
            }),
        }
    }

    fn get(&self, name: &str) -> Result<T> {
        trace!("[get] curr values: {}", self);
        self.values
            .borrow()
            .get(name)
            .cloned()
            .or_else(|| {
                trace!("[get] going into parent...");
                self.parent
                    .as_ref()
                    .and_then(|parent| parent.as_ref().borrow().get(name).ok())
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

#[derive(Debug, Clone, PartialEq)]
pub struct Env<T> {
    current: Link<T>,
    globals: Globals<T>,
}

// impl<T: Clone> Clone for Env<T> {
//     fn clone(&self) -> Self {
//         Self { current: self.current.clone(), globals: self.globals.clone() }
//     }
// }

impl<T: Display + Clone> Env<T> {
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
        if let Some(s) = self.current.take() {
            self.current = s.as_ref().borrow_mut().parent.take();
        }
    }

    pub fn define(&mut self, name: &str, val: T) {
        if let Some(s) = self.current.as_ref() {
            s.as_ref().borrow_mut().define(name, val);
        }
    }

    pub fn assign(&mut self, name: &str, val: T) -> Result<()> {
        self.current
            .as_ref()
            .ok_or(LoxErr::Internal {
                message: "Attempted to assign to an unscoped environment".to_string(),
            })
            .and_then(|s| s.as_ref().borrow_mut().assign(name, val))
    }

    pub fn get(&self, name: &str) -> Result<T> {
        let borrow = self.current.as_ref();
        match borrow {
            Some(current) => {
                let borrow: core::cell::Ref<'_, Scope<T>> = current.as_ref().borrow();
                borrow.get(name)
            }
            None => Err(LoxErr::Internal {
                message: "Attempted to get from an unscoped environment".to_string(),
            }),
        }
    }
}

impl<T: Display> Default for Env<T>
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
        while let Some(link) = curr {
            curr = link.as_ref().borrow_mut().parent.take();
        }
        self.globals.take();
    }
}

impl<T: Display> Display for Env<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt_result {
        match self.current.as_ref() {
            Some(current) => {
                let borrow: core::cell::Ref<'_, Scope<T>> = current.as_ref().borrow();
                write!(f, "Env: {}", borrow.borrow())?;
                Ok(())
            }
            None => Ok(()),
        }
    }
}

impl<T: Display> Display for Scope<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.format_with_indent(f, 0)
    }
}
