use strum_macros::Display as EnumDisplay;

pub type Double = f64;

#[derive(Debug, EnumDisplay)]
pub enum Value {
    Double(Double),
}
