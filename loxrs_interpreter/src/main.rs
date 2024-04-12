mod lox;
use lox::interpreter::start;

fn main() {
    unsafe {
        backtrace_on_stack_overflow::enable();
        env_logger::init();
        start();
    }
}
