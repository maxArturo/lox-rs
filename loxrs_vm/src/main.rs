mod entities;

use entities::opcode::Opcode;

use crate::entities::{chunk::Chunk, value::Value};
fn main() {
    let mut test_chunk = Chunk::new();
    let idx = test_chunk.add_constant(Value::Double(1.3));
    test_chunk.write_chunk(Opcode::Constant(idx));
    test_chunk.write_chunk(Opcode::Return);

    println!("{}", test_chunk);
}
