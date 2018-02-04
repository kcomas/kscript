pub type AstBody = Vec<Vec<Ast>>;

#[derive(Debug, Clone)]
pub enum Ast {
    Comment(String),
    Integer(i64),
    Float(f64),
    VarLocal(usize),
    VarArg(usize),
    Group(AstBody), // ()
    // num arguments
    Function(usize, AstBody),
    LocalFunctionCall(usize, AstBody),
    ArgFunctionCall(usize, AstBody),
    SelfFuctionCall(AstBody),
    IfStatement(AstBody),
    Add,
    Sub,
    Return,
    Assign,
    Equals,
    EqualsGreater,
    EqualsLess,
    Greater,
    Less,
    IoWrite,
    IoAppend,
}

impl Ast {
    pub fn presedence(&self) -> u8 {
        match *self {
            Ast::Comment(_) => 0,
            Ast::Integer(_)
            | Ast::Float(_)
            | Ast::VarLocal(_)
            | Ast::VarArg(_)
            | Ast::Function(_, _) => 1,
            Ast::Return => 2,
            Ast::IfStatement(_) | Ast::Assign | Ast::IoWrite | Ast::IoAppend => 3,
            Ast::Equals | Ast::EqualsGreater | Ast::EqualsLess | Ast::Greater | Ast::Less => 4,
            Ast::Add | Ast::Sub => 5,
            Ast::LocalFunctionCall(_, _) | Ast::ArgFunctionCall(_, _) | Ast::SelfFuctionCall(_) => {
                6
            }
            Ast::Group(_) => 7,
        }
    }
}
