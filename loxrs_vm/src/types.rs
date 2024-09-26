use std::ops::Range;

trait Spanned {
    fn span() -> Range<usize>;
}
