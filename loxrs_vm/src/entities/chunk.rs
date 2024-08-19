use std::ops::Range;

use arrayvec::ArrayVec;

use crate::{
    config::MAX_CONST_POOL,
    error::{LoxError, OverflowError, Result},
    span::Span,
};

use super::{opcode, value::Value};

#[derive(Debug, Default)]
pub struct Chunk {
    code: Vec<u8>,
    constants: ArrayVec<Value, MAX_CONST_POOL>,
    spans: Vec<Span>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: vec![],
            constants: ArrayVec::new(),
            spans: vec![],
        }
    }

    pub fn write_chunk(&mut self, opcode: u8, span: Span) {
        self.code.push(opcode);
        self.spans.push(span)
    }

    fn write_constant(&mut self, val: Value) -> Result<u8> {
        self.constants.try_push(val).map_err(|_| {
            <OverflowError as Into<LoxError>>::into(OverflowError::ExceedsConstSize(MAX_CONST_POOL))
        })?;
        Ok((self.constants.len() - 1)
            .try_into()
            .expect("index overflow exceeds {MAX_CONST_POOL}"))
    }

    pub fn add_constant(&mut self, opcode: u8, val: Value, span: &Span) -> Result<()> {
        let idx = self.write_constant(val)?;
        self.write_chunk(opcode, span.clone());
        self.write_chunk(idx, span.clone());
        Ok(())
    }

    fn display_op(&self, idx: usize) -> usize {
        self.display_span(idx);
        let next_idx = match self.code[idx] {
            opcode::RETURN => self.display_op_simple("OP_RETURN", idx),
            opcode::CONSTANT => self.display_op_one_operand("OP_CONSTANT", idx),
            _byte => self.display_op_simple("OP_UNKNOWN", idx),
        };
        next_idx
    }

    fn display_span(&self, idx: usize) {
        let Range { start, end } = self.spans[idx];
        eprint!(" {start:04}-{end:04}")
    }

    fn display_op_simple(&self, name: &str, idx: usize) -> usize {
        eprintln!("{idx:4}: {name:16}");
        idx + 1
    }

    fn display_op_one_operand(&self, name: &str, idx: usize) -> usize {
        let const_idx = self.code[idx + 1];
        let byte = &self.constants[const_idx as usize];
        eprintln!("{idx:4}: {name:16} -> {}", byte);
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
