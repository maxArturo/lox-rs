# TODOs

- [x] make sure columns are spit out correctly
- [x] add C-style `/* ... */` comments
- [x] comments with columns aren't working correctly
- [x] move out the `pretty-print` capability into `std::fmt::Display`
    to do this you'll need to make an enum that holds multiple kinds of struct variants and implement the display on that. This also removes the need to have a `Expr` trait

