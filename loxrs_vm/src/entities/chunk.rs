use arrayvec::ArrayVec;

use crate::{
    config::MAX_CONST_POOL,
    error::{OverflowError, Result},
};

use super::{opcode, value::Value};

#[derive(Debug, Default)]
pub struct Chunk {
    code: Vec<u8>,
    constants: ArrayVec<Value, MAX_CONST_POOL>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: vec![],
            constants: ArrayVec::new(),
        }
    }

    pub fn write_chunk(&mut self, opcode: u8) {
        self.code.push(opcode)
    }

    pub fn add_constant(&mut self, val: Value) -> Result<u8> {
        self.constants.push(val);
        (self.constants.len() - 1)
            .try_into()
            .map_err(|_| OverflowError::ExceedsConstSize(MAX_CONST_POOL).into())
    }

    fn display_op(&self, idx: usize) -> usize {
        match self.code[idx] {
            opcode::RETURN => self.display_op_simple("OP_RETURN", idx),
            opcode::CONSTANT => self.display_op_one_operand("OP_CONSTANT", idx),
            _byte => self.display_op_simple("OP_UNKNOWN", idx),
        }
    }

    fn display_op_simple(&self, name: &str, idx: usize) -> usize {
        eprintln!("{idx:4}: {name:16}");
        idx + 1
    }

    fn display_op_one_operand(&self, name: &str, idx: usize) -> usize {
        let const_idx = self.code[idx + 1];
        let byte = &self.constants[const_idx as usize];
        eprintln!("{idx:4}: {name:16} -> {:?}", byte);
        idx + 2
    }

    pub fn debug(&self) {
        eprintln!("Chunk");

        let mut idx = 0;
        while idx < self.code.len() {
            idx = self.display_op(idx);
        }
    }
}
