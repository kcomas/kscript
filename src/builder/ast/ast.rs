#[derive(Debug)]
pub enum Ast {
    Comment(String),
    Var(String),
    Bool(bool),
    Integer(i64),
    Float(f64),
    // args, body
    Function(Vec<Ast>, Vec<Ast>),
    // args
    FunctionCall(Vec<Vec<Ast>>),
    Return,
    Assign,
    Equals,
    // body
    If(Vec<Ast>),
    Add,
    Sub,
    IoWrite,
    IoAppend,
}
