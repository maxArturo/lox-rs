use std::{
    fmt::{Display, Formatter},
    ops::Range,
};

use arrayvec::ArrayVec;

use crate::{
    config::MAX_CONST_POOL,
    error::{LoxError, OverflowError, Result as LoxResult},
};

use super::{opcode, value::Value};
type Span = Range<usize>;

#[derive(Debug, Default)]
pub struct Chunk {
    pub code: Vec<u8>,
    constants: ArrayVec<Value, MAX_CONST_POOL>,
    pub spans: Vec<Span>,
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

    fn write_constant(&mut self, val: Value) -> LoxResult<u8> {
        self.constants.try_push(val).map_err(|_| {
            <OverflowError as Into<LoxError>>::into(OverflowError::ExceedsConstSize(MAX_CONST_POOL))
        })?;
        (self.constants.len() - 1).try_into().map_err(|_| {
            <OverflowError as Into<LoxError>>::into(OverflowError::IndexOverflow(MAX_CONST_POOL))
        })
    }

    pub fn add_constant(&mut self, opcode: u8, val: Value, span: &Span) -> LoxResult<()> {
        let idx = self.write_constant(val)?;
        self.write_chunk(opcode, span.clone());
        self.write_chunk(idx, span.clone());
        Ok(())
    }

    pub fn read_const(&self, idx: usize) -> LoxResult<Value> {
        if idx < MAX_CONST_POOL {
            return Ok(self.constants[self.code[idx] as usize]);
        }

        return Err(<OverflowError as Into<LoxError>>::into(
            OverflowError::IndexOverflow(MAX_CONST_POOL),
        ));
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Chunk")?;

        let mut idx = 0;
        while idx < self.code.len() {
            idx = self.display_op(idx, f);
        }
        Ok(())
    }
}

/// display functionality
impl Chunk {
    fn display_op(&self, idx: usize, f: &mut Formatter<'_>) -> usize {
        self.display_span(idx, f).expect("Failed to display span");
        match self.code[idx] {
            opcode::RETURN => self.display_op_simple("OP_RETURN", idx, f),
            opcode::CONSTANT => self.display_op_one_operand("OP_CONSTANT", idx, f),
            opcode::NEGATE => self.display_op_simple("OP_NEGATE", idx, f),
            opcode::ADD => self.display_op_simple("OP_ADD", idx, f),
            opcode::SUBTRACT => self.display_op_simple("OP_SUBTRACT", idx, f),
            opcode::MULTIPLY => self.display_op_simple("OP_MULTIPLY", idx, f),
            opcode::DIVIDE => self.display_op_simple("OP_DIVIDE", idx, f),
            _byte => self.display_op_simple("OP_UNKNOWN", idx, f),
        }
    }

    fn display_span(&self, idx: usize, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Range { start, end } = self.spans[idx];
        write!(f, " {start:04}-{end:04}")?;
        Ok(())
    }

    fn display_op_simple(&self, name: &str, idx: usize, f: &mut Formatter<'_>) -> usize {
        // Return type changed to usize
        writeln!(f, "{idx:4}: {name:16}").expect("Failed to write");
        idx + 1
    }

    fn display_op_one_operand(&self, name: &str, idx: usize, f: &mut Formatter<'_>) -> usize {
        // Return type changed to usize
        let const_idx = self.code[idx + 1];
        let byte = &self.constants[const_idx as usize];
        writeln!(f, "{idx:4}: {name:16} -> {}", byte).expect("Failed to write");
        idx + 2
    }
}
