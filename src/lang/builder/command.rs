
use std::ops::{Add, Sub, Mul, Div, Rem};
use std::collections::HashMap;
use std::fmt;
use super::super::Error;

pub type Kmap = HashMap<String, DataHolder>;

#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    Null,
    Integer(i64),
    Float(f64),
    String(String),
    File(String),
    Bool(bool),
}

impl DataType {
    pub fn get_as_bool(&self) -> bool {
        match *self {
            DataType::Bool(b) => b,
            DataType::Integer(int) => int != 0,
            DataType::Float(float) => float != 0.0,
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match *self {
            DataType::Float(_) => true,
            _ => false,
        }
    }

    pub fn get_as_float(&self) -> f64 {
        match *self {
            DataType::Float(float) => float,
            DataType::Integer(int) => int as f64,
            _ => 0.0,
        }
    }

    pub fn get_as_int(&self) -> i64 {
        match *self {
            DataType::Integer(int) => int,
            DataType::Float(float) => float as i64,
            _ => 0,
        }
    }

    pub fn is_int(&self) -> bool {
        match *self {
            DataType::Integer(_) => true,
            _ => false,
        }
    }

    pub fn write_string(&mut self, to_write: String) -> Result<(), Error> {
        *self = match *self {
            _ => DataType::String(to_write),
        };
        Ok(())
    }

    pub fn append_string(&mut self, to_append: String) -> Result<(), Error> {
        *self = match *self {
            DataType::String(ref self_string) => DataType::String(
                format!("{}{}", self_string, to_append),
            ),
            _ => return Err(Error::UnableToAppendToWrongType),
        };
        Ok(())
    }

    pub fn cast_int(&self) -> Result<i64, Error> {
        match *self {
            DataType::Null => Ok(0),
            DataType::Integer(int) => Ok(int),
            DataType::Float(float) => Ok(float as i64),
            DataType::String(ref string) => {
                match string.parse::<i64>() {
                    Ok(int) => Ok(int),
                    Err(_) => Err(Error::CastFail),
                }
            }
            DataType::File(_) => Err(Error::NYI),
            DataType::Bool(value) => {
                match value {
                    true => Ok(1),
                    false => Ok(0),
                }
            }
        }
    }

    pub fn cast_float(&self) -> Result<f64, Error> {
        match *self {
            DataType::Null => Ok(0.0),
            DataType::Integer(int) => Ok(int as f64),
            DataType::Float(float) => Ok(float),
            DataType::String(ref string) => {
                match string.parse::<f64>() {
                    Ok(float) => Ok(float),
                    Err(_) => Err(Error::CastFail),
                }
            }
            DataType::File(_) => Err(Error::NYI),
            DataType::Bool(value) => {
                match value {
                    true => Ok(1.0),
                    false => Ok(0.0),
                }
            }
        }
    }

    pub fn cast_string(&self) -> Result<String, Error> {
        Ok(format!("{}", self))
    }

    pub fn cast_file(&self) -> Result<(), Error> {
        Err(Error::NYI)
    }

    pub fn cast_bool(&self) -> Result<bool, Error> {
        match *self {
            DataType::Null => Ok(false),
            DataType::Integer(int) => {
                let mut b = false;
                if int > 0 {
                    b = true;
                }
                Ok(b)
            }
            DataType::Float(float) => {
                let mut b = false;
                if float > 0.0 {
                    b = true;
                }
                Ok(b)
            }
            DataType::String(ref string) => {
                let mut b = false;
                if string.len() > 0 {
                    b = true;
                }
                Ok(b)
            }
            DataType::File(_) => Err(Error::NYI),
            DataType::Bool(b) => Ok(b),
        }
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DataType::Null => write!(f, "NULL"),
            DataType::Integer(int) => write!(f, "{}", int),
            DataType::Float(float) => write!(f, "{}", float),
            DataType::String(ref string) => write!(f, "{}", string),
            DataType::File(ref file) => write!(f, "'{}'", file),
            DataType::Bool(boolean) => {
                match boolean {
                    true => write!(f, "t"),
                    false => write!(f, "f"),
                }
            }
        }
    }
}

pub fn coerce_numbers(left: &DataType, right: &DataType) -> (DataType, DataType) {
    if left.is_int() && right.is_int() || left.is_float() && right.is_float() {
        return (left.clone(), right.clone());
    } else if left.is_int() && right.is_float() || left.is_float() && right.is_int() {
        return (
            DataType::Float(left.get_as_float()),
            DataType::Float(right.get_as_float()),
        );
    }
    (
        DataType::Integer(left.get_as_int()),
        DataType::Integer(right.get_as_int()),
    )
}

impl Add for DataType {
    type Output = DataType;

    fn add(self, right: DataType) -> DataType {
        let (left, right) = coerce_numbers(&self, &right);
        if left.is_int() && right.is_int() {
            return DataType::Integer(left.get_as_int() + right.get_as_int());
        }
        DataType::Float(left.get_as_float() + right.get_as_float())
    }
}

impl Sub for DataType {
    type Output = DataType;

    fn sub(self, right: DataType) -> DataType {
        let (left, right) = coerce_numbers(&self, &right);
        if left.is_int() && right.is_int() {
            return DataType::Integer(left.get_as_int() - right.get_as_int());
        }
        DataType::Float(left.get_as_float() - right.get_as_float())
    }
}

impl Mul for DataType {
    type Output = DataType;

    fn mul(self, right: DataType) -> DataType {
        let (left, right) = coerce_numbers(&self, &right);
        if left.is_int() && right.is_int() {
            return DataType::Integer(left.get_as_int() * right.get_as_int());
        }
        DataType::Float(left.get_as_float() * right.get_as_float())
    }
}

impl Div for DataType {
    type Output = DataType;

    fn div(self, right: DataType) -> DataType {
        let (left, right) = coerce_numbers(&self, &right);
        if left.is_int() && right.is_int() {
            return DataType::Integer(left.get_as_int() / right.get_as_int());
        }
        DataType::Float(left.get_as_float() / right.get_as_float())
    }
}

impl Rem for DataType {
    type Output = DataType;

    fn rem(self, right: DataType) -> DataType {
        let (left, right) = coerce_numbers(&self, &right);
        if left.is_int() && right.is_int() {
            return DataType::Integer(left.get_as_int() % right.get_as_int());
        }
        DataType::Float(left.get_as_float() % right.get_as_float())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Comparison {
    Equals,
    NotEquals,
    EqualOrGreater,
    EqualOrLess,
    Greater,
    Less,
    And,
    Or,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CastTo {
    Integer,
    Float,
    String,
    File,
    Bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SystemCommand {
    Exit(u32),
}

#[derive(Debug, PartialEq, Clone)]
pub enum DataHolder {
    Var(String),
    RefVar(String),
    Const(String),
    RefConst(String),
    Anon(DataType),
    Array(Vec<DataHolder>),
    Dict(Kmap),
    ObjectAccess(Box<DataHolder>, Box<DataHolder>),
    // the result in a register
    Math(usize),
    Conditional(Box<DataHolder>, Comparison, Box<DataHolder>),
    Function(Vec<DataHolder>, Vec<Command>),
    FunctionCall(Box<DataHolder>, Vec<DataHolder>),
    System(SystemCommand),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    SetRegister(usize, DataHolder),
    ClearRegisters,
    // left right
    Run(usize, usize),
    Assign(usize, usize),
    TakeReference(usize, usize),
    Dereference(usize, usize),
    IoWrite(usize, usize),
    IoAppend(usize, usize),
    IoRead(usize, usize),
    IoReadAppend(usize, usize),
    // math do op and assigin to new register
    // result left right
    Addition(usize, usize, usize),
    Subtract(usize, usize, usize),
    Multiply(usize, usize, usize),
    Divide(usize, usize, usize),
    Modulus(usize, usize, usize),
    Exponent(usize, usize, usize),
    // coditional true commands false commands
    If(DataHolder, Vec<Command>, Vec<Command>),
    Loop(DataHolder, Vec<Command>),
    Cast(CastTo, usize, usize),
    // array commands
    Len(usize, usize),
}
