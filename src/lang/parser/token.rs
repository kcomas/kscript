
use super::super::builder::command::{DataType, DataHolder};

#[derive(Debug, Clone, PartialEq)]
pub enum SystemCommand {
    Exit(u32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Used,
    End,
    Const(String),
    Var(String),
    Integer(i64),
    Float(f64),
    Assign,
    Run,
    Addition,
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
    String(String),
    Array(Vec<Token>),
    Dict(Vec<Token>, Vec<Token>),
    ObjectAccess(Box<Token>, Box<Token>),
    Bool(bool),
    Equals,
    EqualOrGreater,
    EqualOrLess,
    Greater,
    Less,
    And,
    Or,
    Conditional(Box<Token>, Box<Token>, Box<Token>),
    If(Box<Token>, Vec<Token>, Vec<Token>),
    Loop(Box<Token>, Vec<Token>),
    Ref(Box<Token>),
    Function(Vec<Token>, Vec<Token>),
    FunctionCall(Box<Token>, Vec<Token>),
    System(SystemCommand),
    // used to denote value in register
    Reg(usize),
}

impl Token {
    pub fn is_end(&self) -> bool {
        match *self {
            Token::End => true,
            _ => false,
        }
    }

    pub fn is_operator_with_presedence(&self) -> u64 {
        // if it is greater then zero it is an operator
        // higher number is higher presedene
        match *self {
            Token::Assign | Token::IoWrite => 1,
            Token::Run => 2,
            _ => 0,
        }
    }

    pub fn set_as_register(&mut self, reg_counter: usize) {
        *self = Token::Reg(reg_counter);
    }

    pub fn is_register(&self) -> Option<usize> {
        match *self {
            Token::Reg(reg_counter) => Some(reg_counter),
            _ => None,
        }
    }

    pub fn to_data_holder(&self) -> Option<DataHolder> {
        match *self {
            Token::Var(ref name) => Some(DataHolder::Var(name.clone())),
            Token::Const(ref name) => Some(DataHolder::Const(name.clone())),
            Token::String(ref string) => Some(DataHolder::Anon(DataType::String(string.clone()))),
            Token::Integer(int) => Some(DataHolder::Anon(DataType::Integer(int))),
            Token::Float(float) => Some(DataHolder::Anon(DataType::Float(float))),
            Token::Array(ref arr) => {
                let mut container: Vec<DataHolder> = Vec::new();
                for token in arr.iter() {
                    if let Some(item) = token.to_data_holder() {
                        container.push(item);
                    }
                }
                Some(DataHolder::Array(container))
            }
            Token::ObjectAccess(ref target, ref accessor) => {
                let t_holder = target.to_data_holder();
                let a_holder = accessor.to_data_holder();
                if t_holder.is_some() && a_holder.is_some() {
                    return Some(DataHolder::ObjectAccess(
                        Box::new(t_holder.unwrap()),
                        Box::new(a_holder.unwrap()),
                    ));
                }
                None
            }
            _ => None,
        }
    }
}
