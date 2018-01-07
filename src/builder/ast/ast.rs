#[derive(Debug)]
pub enum Ast {
    End,
    Comment(String),
    Var(String),
    Bool(bool),
    Integer(i64),
    Float(f64),
    // args, body
    Function(Vec<Vec<Vec<Ast>>>, Vec<Vec<Ast>>),
    // args
    FunctionCall(Vec<Vec<Vec<Ast>>>),
    // body
    If(Vec<Vec<Ast>>),
    Return,
    Assign,
    Equals,
    Add,
    Sub,
    IoWrite,
    IoAppend,
}

impl Ast {
    pub fn add_end(&self) -> bool {
        match *self {
            Ast::Function(_, _) | Ast::If(_) => true,
            _ => false,
        }
    }
}
