
use super::super::builder::command::Comparison;

#[derive(Debug, PartialEq)]
pub enum SystemCommand {
    Exit(u32),
}

#[derive(Debug, PartialEq)]
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

    pub fn to_comparison(&self) -> Option<Comparison> {
        match *self {
            Token::Equals => Some(Comparison::Equals),
            Token::EqualOrGreater => Some(Comparison::EqualOrGreater),
            Token::EqualOrLess => Some(Comparison::EqualOrLess),
            Token::Greater => Some(Comparison::Greater),
            Token::Less => Some(Comparison::Less),
            Token::And => Some(Comparison::And),
            Token::Or => Some(Comparison::Or),
            _ => None,
        }
    }
}
