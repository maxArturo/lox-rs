mod config;
mod entities;
mod error;

use crate::entities::{chunk::Chunk, opcode, value::Value};
fn main() {
    let mut test_chunk = Chunk::new();
    let idx = test_chunk.add_constant(Value::from(1.2)).unwrap();
    test_chunk.write_chunk(opcode::CONSTANT);
    test_chunk.write_chunk(idx);
    test_chunk.write_chunk(opcode::RETURN);

    test_chunk.debug();
}
