use iota::iota;

iota! {
    pub const RETURN: u8 = iota;,
    CONSTANT,
    NEGATE,
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    TERNARY_LOGICAL,
    NOT,
    GREATER,
    EQUAL,
    LESS
}
