mod config;
mod entities;
mod error;
mod span;

use crate::entities::{chunk::Chunk, opcode, value::Value};
fn main() {
    let mut test_chunk = Chunk::new();
    test_chunk
        .add_constant(opcode::CONSTANT, Value::from(1.2), &(1..4))
        .unwrap();
    test_chunk.write_chunk(opcode::RETURN, 5..6);
    test_chunk
        .add_constant(opcode::CONSTANT, Value::from(1.2), &(7..40))
        .unwrap();

    test_chunk.write_chunk(opcode::RETURN, 41..42);

    test_chunk.debug();
}
