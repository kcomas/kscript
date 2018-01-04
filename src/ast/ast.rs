use super::super::data_type::DataType;
use super::super::error::Error;

#[derive(Debug, Clone)]
pub enum Ast {
    Used,
    UsedVar(String),
    Comment(String),
    End,
    Var(String),
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<Vec<Ast>>),
    Access(Vec<Ast>),
    ArrayPush,
    ArrayPop,
    Group(Vec<Ast>), // (...)
    // var, args, body
    Function(Box<Ast>, Vec<Vec<Ast>>, Vec<Ast>),
    If(Vec<Ast>),
    Assign,
    Equals,
    Add,
    Sub,
    Mul,
    Exp,
    Div,
    Rem,
    IoWrite,
    IoAppend,
    Return,
}

impl<'a> Ast {
    pub fn presedence(&self) -> usize {
        if self.is_data() || self.is_array() || self.is_group() {
            return 1;
        }
        match *self {
            Ast::Return => 2,
            Ast::IoWrite | Ast::IoAppend | Ast::Assign | Ast::ArrayPush | Ast::ArrayPop => 3,
            Ast::If(_) => 4,
            Ast::Equals => 5,
            Ast::Add | Ast::Sub => 6,
            Ast::Mul | Ast::Div | Ast::Rem => 7,
            Ast::Exp => 8,
            Ast::Function(_, _, _) => 9,
            Ast::Access(_) => 10,
            _ => 0,
        }
    }

    pub fn is_end(&self) -> bool {
        if let Ast::End = *self {
            return true;
        }
        false
    }

    pub fn is_used(&self) -> bool {
        match *self {
            Ast::Used | Ast::UsedVar(_) => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        if let Ast::Bool(_) = *self {
            return true;
        }
        false
    }

    pub fn is_number(&self) -> bool {
        match *self {
            Ast::Integer(_) | Ast::Float(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        if let Ast::String(_) = *self {
            return true;
        }
        false
    }

    pub fn is_data(&self) -> bool {
        self.is_var() || self.is_number() || self.is_string() || self.is_bool()
    }

    pub fn is_var(&self) -> bool {
        match *self {
            Ast::Var(_) => true,
            _ => false,
        }
    }

    pub fn is_used_var(&self) -> bool {
        match *self {
            Ast::UsedVar(_) => true,
            _ => false,
        }
    }

    pub fn get_var_name(&self) -> Result<&str, Error<'a>> {
        match *self {
            Ast::Var(ref name) | Ast::UsedVar(ref name) => Ok(name),
            _ => Err(Error::AstNotVar("Token not a var")),
        }
    }

    pub fn is_group(&self) -> bool {
        if let Ast::Group(_) = *self {
            return true;
        }
        false
    }

    pub fn get_group_body_mut(&mut self) -> Result<&mut Vec<Ast>, Error<'a>> {
        if let Ast::Group(ref mut body) = *self {
            return Ok(body);
        }
        Err(Error::InvalidGroup("Not a group"))
    }

    pub fn is_array(&self) -> bool {
        if let Ast::Array(_) = *self {
            return true;
        }
        false
    }

    pub fn get_array_body_mut(&mut self) -> Result<&mut Vec<Vec<Ast>>, Error<'a>> {
        match *self {
            Ast::Array(ref mut body) => Ok(body),
            _ => Err(Error::InvalidArray("Ast is not an array")),
        }
    }

    pub fn is_access(&self) -> bool {
        if let Ast::Access(_) = *self {
            return true;
        }
        false
    }

    pub fn get_access_body_mut(&mut self) -> Result<&mut Vec<Ast>, Error<'a>> {
        match *self {
            Ast::Access(ref mut body) => Ok(body),
            _ => Err(Error::InvalidAccess("Ast is not an access")),
        }
    }

    pub fn is_function(&self) -> bool {
        match *self {
            Ast::Function(_, _, _) => true,
            _ => false,
        }
    }

    pub fn is_function_def(&self) -> bool {
        match *self {
            Ast::Function(_, _, ref body) => {
                for item in body.iter() {
                    if !item.is_end() {
                        return true;
                    }
                }
                false
            }
            _ => false,
        }
    }

    pub fn get_function_name(&self) -> Result<&str, Error<'a>> {
        match *self {
            Ast::Function(ref name_token, _, _) => match **name_token {
                Ast::Var(ref name) => Ok(name),
                _ => Err(Error::AstIsNotAFunction("Function name is not a var")),
            },
            _ => Err(Error::AstIsNotAFunction("Not a function cannot get name")),
        }
    }

    pub fn get_function_args(&self) -> Result<&Vec<Vec<Ast>>, Error<'a>> {
        match *self {
            Ast::Function(_, ref args, _) => Ok(args),
            _ => Err(Error::AstIsNotAFunction(
                "Not a function cannot get function args",
            )),
        }
    }

    pub fn get_function_args_mut(&mut self) -> Result<&mut Vec<Vec<Ast>>, Error<'a>> {
        match *self {
            Ast::Function(_, ref mut args, _) => Ok(args),
            _ => Err(Error::AstIsNotAFunction(
                "Not a function cannot get function args mut",
            )),
        }
    }

    pub fn get_function_body_mut(&mut self) -> Result<&mut Vec<Ast>, Error<'a>> {
        match *self {
            Ast::Function(_, _, ref mut body) => Ok(body),
            _ => Err(Error::AstIsNotAFunction(
                "Not a function cannot get function body",
            )),
        }
    }

    // have to specify
    pub fn is_dyadic(&self) -> bool {
        match *self {
            Ast::ArrayPush
            | Ast::ArrayPop
            | Ast::Assign
            | Ast::Equals
            | Ast::Add
            | Ast::Sub
            | Ast::Mul
            | Ast::Exp
            | Ast::Div
            | Ast::Rem
            | Ast::IoWrite
            | Ast::IoAppend => true,
            _ => false,
        }
    }

    pub fn is_monadic_left(&self) -> bool {
        match *self {
            Ast::Return => true,
            Ast::Access(_) => true,
            _ => false,
        }
    }

    pub fn to_data_type(&self) -> Result<DataType, Error<'a>> {
        match *self {
            Ast::Bool(b) => Ok(DataType::Bool(b)),
            Ast::Integer(int) => Ok(DataType::Integer(int)),
            Ast::Float(float) => Ok(DataType::Float(float)),
            Ast::String(ref string) => Ok(DataType::String(string.clone())),
            _ => Err(Error::CannotConvertToDataType(
                self.clone(),
                "Cannot convert to data type",
            )),
        }
    }

    pub fn is_if(&self) -> bool {
        if let Ast::If(_) = *self {
            return true;
        }
        false
    }

    pub fn get_if_body_mut(&mut self) -> Result<&mut Vec<Ast>, Error<'a>> {
        if let Ast::If(ref mut body) = *self {
            return Ok(body);
        }
        Err(Error::CannotGetIfBody("Not an if"))
    }

    pub fn is_assign(&self) -> bool {
        if let Ast::Assign = *self {
            return true;
        }
        false
    }
}
