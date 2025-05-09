# TODOs

- [x] make sure columns are spit out correctly
- [x] add C-style `/* ... */` comments
- [x] comments with columns aren't working correctly
- [x] move out the `pretty-print` capability into `std::fmt::Display`
- [x] [BUG] running `print """;` in REPL yields error, e.g.:
```shell
This is the LOX interpreter.
Enter statements separated by ENTER.
EXIT with CTRL-D.
> print """;
suposed to print here
[2024-03-08T10:01:04Z DEBUG lox_rs::lox::interpreter::scanner] received: print """;
    
thread 'main' panicked at src/lox/interpreter/scanner.rs:288:9:
attempt to subtract with overflow

```
- [x] make persistent env and interpreter for REPL so that variables continue living
- [x] add git precommit hooks locally
- [x] add `rustfmt` gh action for the project
- [x] ensure env cloning is correct and performant
- [x] use single source of truth for named function declaration and IIFE
- [x] prevent reassignment of shadow variables
- [x] working variable resolution pass
- [x] error on reassigned variables within same scope
- [x] error on return statements outside of function scope
- [x] 2024-07-08 08:20 need to see how to store references to classes from an instance in parsing
- [x] fix binary comparisons between instances/classes 

## nice to haves

- [x] escape sequences like this
```shell
> print "\"";
==> "
```


