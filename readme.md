# lox-rs

A quick n dirty implementation of the Lox language, according to the book [Crafting Interpreters](https://craftinginterpreters.com/) in rust.

However, this doesn't mean its quick (compilers certainly aren't!) or dirty (Rust won't allow it, and also all the tests according to Munificent's spec pass). This is my cutting-teeth intro to Rust, from real code and first principles. All omissions and errors are mine.

The code samples (which are also e2e tests) are located [here](./loxrs_interpreter/src/lox/interpreter/test/e2e/). There are examples of both valid and invalid `lox` code.

You can see a list of [TODOs](./todo.md) as well.

running a single lox file

```shell
cargo run -- ./loxrs_interpreter/src/lox/interpreter/test/e2e/unimplemented/basic_class.lox

```

for running the VM version: 

```shell
RUST_BACKTRACE=1 RUST_LOG=trace cargo run --bin loxrs_vm -- ./loxrs_interpreter/src/lox/interpreter/test/e2e/pass/simple.lox
```

or a REPL for the VM: 
```shell
RUST_LOG=trace cargo run --bin loxrs_vm
```

## Performance

Running the `loxrs_interpreter/src/lox/interpreter/test/e2e/spec/benchmark/fib.lox` on the `release` build of the treewalk interpreter on a 2.6 GHz 6-Core Intel Core i7 outputs:

```
==> true
==> 189.5220010280609
```

The bytecode interpreter ought to be much faster.
