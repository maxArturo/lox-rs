use std::ops::Range;

pub trait Spanned {
    fn span(&self) -> Range<usize>;
}
pub type Span = Range<usize>;
