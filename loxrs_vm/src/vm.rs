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
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            stack: ArrayVec::new(),
        }
    }

    pub fn interpret(&mut self) -> Result<()> {
        self.run()
    }

    pub fn run(&mut self) -> Result<()> {
        let mut idx = 0;
        loop {
            trace!("{self}");

            match self.chunk.code[idx] {
                opcode::RETURN => {
                    let val = self.stack.pop().ok_or_else(|| {
                        <InvalidAccessError as Into<LoxError>>::into(
                            InvalidAccessError::StackEmpty.into(),
                        )
                    })?;

                    println!("RETURN VALUE: {val}");
                    break;
                }
                opcode::CONSTANT => {
                    let constant = self.chunk.get_constant(self.chunk.code[idx + 1] as usize)?;
                    self.stack.push(constant);
                    idx += 2;
                }
                opcode::NEGATE => {
                    let val = self
                        .stack
                        .pop()
                        .ok_or_else(|| InvalidAccessError::StackEmpty.into())
                        .and_then(|el| el.try_number())?;
                    self.stack.push(Value::from(-val));
                    idx += 1;
                }
                other => return Err(InternalError::UnknownOperation(other).into()),
            }
        }
        Ok(())
    }
}

impl Display for VM {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "VM <")?;

        writeln!(f, "Stack: [")?;
        for (i, value) in self.stack.iter().enumerate() {
            writeln!(f, "{i:04} [{:p}]: {value}", value)?; // Add a comma before the next element
        }
        write!(f, "]>")
    }
}
