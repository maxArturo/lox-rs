use strum_macros::Display;

#[derive(Debug, Display)]
pub enum Opcode {
    Return,
    Constant(usize),
}
