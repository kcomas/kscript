#[derive(Debug)]
pub enum Ast {
    End,
    Comment(String),
    Var(String),
    Integer(i64),
    // args, body
    Function(Vec<Ast>, Vec<Ast>),
    // args
    SelfFunctionCall(Vec<Vec<Ast>>),
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
