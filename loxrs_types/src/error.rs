use std::fmt;

pub type Result<T, U = LoxErr> = std::result::Result<T, U>;

#[derive(Debug, Clone)]
pub enum LoxErr {
    Undefined {
        message: String,
    },
    Eval {
        expr: String,
        message: String,
    },
    Internal {
        message: String,
    },
    Resolve {
        message: String,
    },
    Parse {
        token: String,
        line: String,
        column: String,
    },
    Scan {
        line: i32,
        col: i32,
        message: String,
    },
}

impl fmt::Display for LoxErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Parse {
                    token,
                    line,
                    column,
                } => format!(
                    "Parsing error for token: {}\nat line: {}, col: {}",
                    token, line, column
                ),
                Self::Eval { expr, message } => {
                    format!("Eval error: {}\n for expression of type: {}", expr, message)
                }
                Self::Scan { line, col, message } => {
                    format!("Syntax error: {}\nat line: {}, col: {}", message, line, col)
                }
                Self::Internal { message } => format!("Internal program error: {}", message),
                Self::Undefined { message } => format!("Undefined error: {}", message),

                Self::Resolve { message } => format!("Variable resolving error: {}", message),
            }
        )
    }
}
