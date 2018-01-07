#[derive(Debug)]
pub enum Ast {
    Comment(String),
    Var(String),
    Bool(bool),
    Integer(i64),
    Float(f64),
    // args, body
    Function(Vec<Vec<Ast>>, Vec<Ast>),
    // args
    FunctionCall(Vec<Vec<Ast>>),
    // body
    If(Vec<Ast>),
    Return,
    Assign,
    Equals,
    Add,
    Sub,
    IoWrite,
    IoAppend,
}
