
#[derive(Debug, Clone)]
pub enum Token {
    Constant(String),
    Var(String),
    Integer(i64),
    Float(f64),
    Assign,
}
