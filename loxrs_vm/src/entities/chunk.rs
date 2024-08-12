use std::fmt::Display;

use super::{opcode::Opcode, value::Value};

#[derive(Debug)]
pub struct Chunk {
    code: Vec<Opcode>,
    constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: vec![],
            constants: vec![],
        }
    }

    pub fn write_chunk(&mut self, opcode: Opcode) {
        self.code.push(opcode)
    }

    pub fn add_constant(&mut self, val: Value) -> usize {
        self.constants.push(val);
        self.constants.len() - 1
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("Chunk");
        for (index, opcode) in self.code.iter().enumerate() {
            ds.field(&format!("{:04}", index), opcode);
        }
        for (index, val) in self.constants.iter().enumerate() {
            ds.field(&format!("constant {}", index), val);
        }
        ds.finish()
    }
}
