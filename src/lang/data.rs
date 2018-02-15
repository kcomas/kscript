use super::function::FunctionPointer;

#[derive(Debug)]
pub enum DataHolder {
    Bool(bool),
    Integer(i64),
    Float(f64),
    Char(char),
    Function(FunctionPointer),
    String(String),
}

#[derive(Debug)]
pub enum RefHolder<'a> {
    Bool(bool),
    Integer(i64),
    Float(f64),
    Char(char),
    Function(FunctionPointer),
    String(&'a String),
}
