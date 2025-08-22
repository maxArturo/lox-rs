use core::f64;
use std::{
    fmt::{Display, Formatter},
    sync::OnceLock,
};

use arrayvec::ArrayVec;
use log::trace;

use crate::{
    compiler::compile,
    config::MAX_STACK,
    entities::{
        chunk::Chunk,
        object::{ObjString, Object},
        opcode,
        value::Value,
    },
    error::{InternalError, InvalidAccessError, LoxError, LoxErrorS, Result},
};

use std::sync::Mutex;

static STRING_POOL: OnceLock<Mutex<Vec<String>>> = OnceLock::new();

// TODO manage memory at this level via GC
pub fn intern_string(s: &str) -> &'static str {
    let pool = STRING_POOL.get_or_init(|| Mutex::new(Vec::new()));
    let mut pool = pool.lock().unwrap();

    // Check if string already exists
    for existing in pool.iter() {
        if existing == s {
            return unsafe { std::mem::transmute(existing.as_str()) };
        }
    }

    // Add new string
    pool.push(s.to_string());
    let last = pool.last().unwrap();
    unsafe { std::mem::transmute(last.as_str()) }
}

#[derive(Debug)]
pub struct VM {
    chunk: Chunk,
    stack: ArrayVec<Value, MAX_STACK>,
    pub ip: usize,
}

impl VM {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::default(),
            stack: ArrayVec::new(),
            ip: 0,
        }
    }

    pub fn interpret(&mut self, source: &str) -> Result<(), Vec<LoxErrorS>> {
        self.chunk = compile(source)?;
        trace!("interpreting VM chunk: {}", self.chunk);
        match self.run() {
            Err(err) => Err(vec![(err, self.chunk.spans[self.ip].clone())]),
            Ok(()) => Ok(()),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            trace!("chunk idx at: {self}");
            let ip = self.ip();
            match self.chunk.code[ip] {
                opcode::RETURN => {
                    let val = self.try_pop()?;

                    println!("RETURN VALUE: {val}");
                    break;
                }
                opcode::CONSTANT => self.constant()?,
                opcode::NOT => self.not()?,
                opcode::NEGATE => self.negate()?,
                opcode::ADD => {
                    let b = self.try_pop()?;
                    let a = self.last_mut()?;

                    if a.is_str() && b.is_str() {
                        trace!("both are strings");

                        // TODO needs drop/alloc
                        let res = unsafe {
                            [
                                (*(a.as_object()).string).value,
                                (*(b.as_object()).string).value,
                            ]
                            .concat()
                        };
                        let str = intern_string(&*res);
                        let object = Box::into_raw(Box::new(ObjString::new(str)));

                        trace!("resulting string: {}", str);
                        *a = Value::from(<*mut ObjString as Into<Object>>::into(object));

                        trace!("finalized value: {}", a);
                        continue;
                    }

                    let b = b.try_number()?;
                    *a = Value::from(a.try_number()? + b);
                }
                opcode::SUBTRACT => self.binary_op_number(|a, b| a - b)?,
                opcode::MULTIPLY => self.binary_op_number(|a, b| a * b)?,
                opcode::DIVIDE => self.binary_op_number(|a, b| a / b)?,
                opcode::GREATER => self.binary_op_number(|a, b| a > b)?,
                opcode::LESS => self.binary_op_number(|a, b| a < b)?,
                opcode::EQUAL => self.equal()?,
                opcode::TERNARY_LOGICAL => self.ternary_op_number(|tern, a, b| match tern {
                    true => a,
                    false => b,
                })?,
                other => return Err(InternalError::UnknownOperation(other).into()),
            }
        }
        Ok(())
    }

    fn ip(&mut self) -> usize {
        let ip = self.ip;
        self.ip += 1;
        ip
    }

    fn negate(&mut self) -> Result<(), LoxError> {
        if let Some(last) = self.stack.last_mut() {
            let num: f64 = -(last.try_number()?);
            *last = Value::from(num);
            Ok(())
        } else {
            return Err(InvalidAccessError::StackEmpty.into());
        }
    }

    fn not(&mut self) -> Result<(), LoxError> {
        if let Some(last) = self.stack.last_mut() {
            *last = Value::from(last.is_falsey());
            Ok(())
        } else {
            return Err(InvalidAccessError::StackEmpty.into());
        }
    }

    fn constant(&mut self) -> Result<()> {
        let val = self.read_const()?;
        self.stack.push(val);
        Ok(())
    }

    fn read_const(&mut self) -> Result<Value> {
        let ip = self.ip();
        self.chunk.read_const(ip)
    }

    fn try_pop(&mut self) -> Result<Value> {
        self.stack.pop().ok_or_else(|| {
            <InvalidAccessError as Into<LoxError>>::into(InvalidAccessError::StackEmpty.into())
        })
    }

    fn ternary_op_number<F: FnOnce(bool, f64, f64) -> f64>(&mut self, op: F) -> Result<()> {
        // Stack is LIFO so we reverse the order
        let c = self.try_pop()?.try_number()?;
        let b = self.try_pop()?.try_number()?;

        if let Some(a) = self.stack.last_mut() {
            *a = Value::from(op(a.try_bool()?, b, c));
            Ok(())
        } else {
            return Err(InvalidAccessError::StackEmpty.into());
        }
    }

    fn last_mut(&mut self) -> Result<&mut Value> {
        if let Some(a) = self.stack.last_mut() {
            Ok(a)
        } else {
            return Err(InvalidAccessError::StackEmpty.into());
        }
    }

    fn equal(&mut self) -> Result<()> {
        let b = self.try_pop()?;
        let a = self.last_mut()?;

        if a.is_object() && b.is_object() {
            trace!("both are objects, comparing equality");

            trace!("a: {:?}, b: {:?}", a.as_object(), b.as_object());
            *a = Value::from(a.as_object() == b.as_object());
        } else {
            *a = Value::from(*a == b);
        }

        Ok(())
    }

    fn binary_op_number<T, F: FnOnce(f64, f64) -> T>(&mut self, op: F) -> Result<()>
    where
        Value: From<T>,
    {
        // Stack is LIFO so we reverse the order
        let b = self.try_pop()?.try_number()?;
        let a = self.last_mut()?;
        *a = Value::from(op(a.try_number()?, b));
        Ok(())
    }
}

impl Display for VM {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(f, "VM <pointer: {:04}>", self.ip)?;
        writeln!(f, "Stack:")?;
        for (i, value) in self.stack.iter().enumerate() {
            writeln!(f, "{i:04} [{:p}]: {value}", value)?; // Add a comma before the next element
        }
        Ok(())
    }
}
