
use std::collections::HashMap;

pub type Kmap = HashMap<String, DataHolder>;

#[derive(Debug, PartialEq)]
pub enum DataType {
    Integer(i64),
    Float(f64),
    String(String),
    File(String),
    Bool(bool),
}

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equals,
    EqualOrGreater,
    EqualOrLess,
    Greater,
    Less,
    And,
    Or,
}

#[derive(Debug, PartialEq)]
pub enum DataHolder {
    Var(String),
    Const(String),
    Anon(DataType),
    Array(Vec<DataHolder>),
    Dict(Kmap),
    ObjectAccess(Box<DataHolder>, Box<DataHolder>),
    // the result in a register
    Math(usize),
    Conditional(Box<DataHolder>, Comparison, Box<DataHolder>),
}

#[derive(Debug, PartialEq)]
pub enum Command {
    SetRegister(usize, DataHolder),
    ClearRegisters,
    // left right
    Run(usize, usize),
    Assign(usize, usize),
    IoWrite(usize, usize),
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
}
