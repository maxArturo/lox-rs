mod lox;
use lox::interpreter::start;

fn main() {
    env_logger::init();
    start();
}
