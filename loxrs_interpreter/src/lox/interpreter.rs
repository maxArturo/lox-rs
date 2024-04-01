mod eval;
mod fns;
mod input;
mod parser;
mod reader;
mod scanner;

pub(crate) use input::read_input as start;
pub(super) use scanner::scan_parse;
