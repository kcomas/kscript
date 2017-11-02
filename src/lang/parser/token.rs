
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    End,
    Const(String),
    Var(String),
    Integer(i64),
    Float(f64),
    Assign,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Exponent,
    Math(Vec<Token>),
    IoWrite,
    IoAppend,
    IoRead,
    IoReadAppend,
    Comment(String),
    File(String),
}
