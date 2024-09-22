use std::fmt::{Display, Formatter};

use arrayvec::ArrayVec;
use log::trace;

use crate::{
    config::MAX_STACK,
    entities::{chunk::Chunk, opcode, value::Value},
    error::{InternalError, InvalidAccessError, LoxError, Result},
};

#[derive(Debug)]
pub struct VM {
    chunk: Chunk,
    stack: ArrayVec<Value, MAX_STACK>,
    ip: usize,
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            stack: ArrayVec::new(),
            ip: 0,
        }
    }

    pub fn interpret(&mut self) -> Result<()> {
        self.run()
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
                opcode::NEGATE => self.negate()?,
                opcode::ADD => self.binary_op_number(|a, b| a + b)?,
                opcode::SUBTRACT => self.binary_op_number(|a, b| a - b)?,
                opcode::MULTIPLY => self.binary_op_number(|a, b| a * b)?,
                opcode::DIVIDE => self.binary_op_number(|a, b| a / b)?,
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

    fn negate(&mut self) -> Result<()> {
        let last = self.stack.len() - 1;
        self.stack[last] = Value::from(-(self.stack[last].try_number()?));
        Ok(())
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

    fn binary_op_number<F: FnOnce(f64, f64) -> f64>(&mut self, op: F) -> Result<()> {
        // Stack is LIFO so we reverse the order
        let b = self.try_pop()?.try_number()?;
        let a = self.try_pop()?.try_number()?;
        self.stack.push(Value::from(op(a, b)));
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
