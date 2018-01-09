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

    pub fn presedence(&self) -> usize {
        match *self {
            Ast::End | Ast::Comment(_) => 0,
            Ast::Var(_) | Ast::Bool(_) | Ast::Integer(_) | Ast::Float(_) | Ast::Function(_, _) => 1,
            Ast::Assign | Ast::Return | Ast::IoWrite | Ast::IoAppend => 2,
            Ast::If(_) | Ast::FunctionCall(_) => 3,
            Ast::Equals => 4,
            Ast::Add | Ast::Sub => 5,
        }
    }

    pub fn has_body(&self) -> bool {
        match *self {
            Ast::If(_) | Ast::Function(_, _) | Ast::FunctionCall(_) => true,
            _ => false,
        }
    }
}
