# lox-rs

A quick n dirty implementation of the Lox language, according to the book [Crafting Interpreters](https://craftinginterpreters.com/) in rust.

This is my cutting-teeth intro to Rust, from real code and first principles. All omissions and errors are mine.

The code samples (which are also e2e tests) are located [here](./loxrs_interpreter/src/lox/interpreter/test/e2e/). There are examples of both valid and invalid `lox` code.

You can see a list of [TODOs](./todo.md) as well.

running a single lox file

```shell
cargo run -- ./loxrs_interpreter/src/lox/interpreter/test/e2e/unimplemented/basic_class.lox

```

## Performance

Running the `loxrs_interpreter/src/lox/interpreter/test/e2e/spec/benchmark/fib.lox` on the `release` build of the treewalk interpreter on a 2.6 GHz 6-Core Intel Core i7 outputs:

```
==> true
==> 189.5220010280609
```
