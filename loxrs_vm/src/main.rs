mod config;
mod entities;
mod error;
mod span;
mod vm;

use log::trace;
use vm::VM;

use crate::entities::{chunk::Chunk, opcode, value::Value};
fn main() {
    env_logger::init();
    let mut test_chunk = Chunk::new();
    test_chunk
        .add_constant(opcode::CONSTANT, Value::from(1.0), &(1..4))
        .unwrap();

    test_chunk
        .add_constant(opcode::CONSTANT, Value::from(2.0), &(1..4))
        .unwrap();

    test_chunk
        .add_constant(opcode::CONSTANT, Value::from(3.0), &(1..4))
        .unwrap();

    test_chunk.write_chunk(opcode::MULTIPLY, 41..42);

    test_chunk.write_chunk(opcode::ADD, 41..42);

    test_chunk
        .add_constant(opcode::CONSTANT, Value::from(4.0), &(1..4))
        .unwrap();

    test_chunk.write_chunk(opcode::SUBTRACT, 41..42);

    test_chunk
        .add_constant(opcode::CONSTANT, Value::from(5.0), &(1..4))
        .unwrap();

    test_chunk.write_chunk(opcode::NEGATE, 42..43);
    test_chunk.write_chunk(opcode::DIVIDE, 42..43);
    test_chunk.write_chunk(opcode::RETURN, 42..43);

    trace!("chunk: {test_chunk}");
    let mut vm = VM::new(test_chunk);

    let _ = vm.interpret().inspect_err(|e| println!("{e}"));
}
