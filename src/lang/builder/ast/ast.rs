pub type AstArgs = Vec<Vec<Vec<Ast>>>;

pub type AstBody = Vec<Vec<Ast>>;

#[derive(Debug, Clone)]
pub enum Ast {
    End,
    Comment(String),
    Var(String),
    VarArg(String, usize),
    VarLocal(String, usize),
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Group(AstBody),
    // args, body
    Function(AstArgs, AstBody),
    // args
    FunctionCall(AstArgs),
    // body
    If(AstBody),
    Return,
    Assign(AstBody),
    Equals,
    Add,
    Concat,
    Sub,
    Mul,
    Div,
    Rem,
    Exp,
    IoWrite,
    IoAppend,
}

impl Ast {
    pub fn presedence(&self) -> usize {
        match *self {
            Ast::End | Ast::Comment(_) => 0,
            Ast::Var(_)
            | Ast::VarLocal(_, _)
            | Ast::VarArg(_, _)
            | Ast::Bool(_)
            | Ast::Integer(_)
            | Ast::Float(_)
            | Ast::String(_)
            | Ast::Function(_, _) => 1,
            Ast::If(_) | Ast::Assign(_) | Ast::Return | Ast::IoWrite | Ast::IoAppend => 2,
            Ast::Equals | Ast::Concat => 3,
            Ast::Add | Ast::Sub => 4,
            Ast::Mul | Ast::Div | Ast::Rem => 5,
            Ast::Exp => 6,
            Ast::FunctionCall(_) => 7,
            Ast::Group(_) => 8,
        }
    }

    pub fn has_var_name(&self) -> Option<&str> {
        if let Ast::Var(ref name) = *self {
            return Some(name);
        }
        None
    }

    pub fn is_var(&self) -> bool {
        match *self {
            Ast::Var(_) | Ast::VarArg(_, _) | Ast::VarLocal(_, _) => true,
            _ => false,
        }
    }

    pub fn num_look_back(&self) -> usize {
        match *self {
            Ast::Assign(_) => 1,
            _ => 0,
        }
    }

    pub fn is_group(&self) -> Option<&AstBody> {
        if let Ast::Group(ref body) = *self {
            return Some(body);
        }
        None
    }

    pub fn is_function_call(&self) -> Option<&AstArgs> {
        if let Ast::FunctionCall(ref body) = *self {
            return Some(body);
        }
        None
    }

    pub fn is_if(&self) -> Option<&AstBody> {
        if let Ast::If(ref body) = *self {
            return Some(body);
        }
        None
    }

    pub fn is_assign(&self) -> Option<&AstBody> {
        if let Ast::Assign(ref body) = *self {
            return Some(body);
        }
        None
    }

    pub fn is_data(&self) -> bool {
        match *self {
            Ast::Bool(_)
            | Ast::Integer(_)
            | Ast::Float(_)
            | Ast::String(_)
            | Ast::Function(_, _) => true,
            _ => false,
        }
    }

    pub fn can_call(&self) -> bool {
        match *self {
            Ast::VarArg(_, _) | Ast::VarLocal(_, _) | Ast::Function(_, _) => true,
            _ => false,
        }
    }
}
