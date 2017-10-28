
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    End,
    Constant(String),
    Var(String),
    Integer(i64),
    Float(f64),
    Assign,
}
