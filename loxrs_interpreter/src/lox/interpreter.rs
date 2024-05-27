mod eval;
mod func;
mod input;
mod parser;
mod reader;
mod resolver;
mod scanner;
mod visitor;

#[cfg(test)]
mod test;

pub(crate) use input::read_input as start;
pub(super) use scanner::scan_parse;
