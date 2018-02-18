use super::function::FunctionPointer;
use super::error::RuntimeError;

#[derive(Debug)]
pub enum Data {
    Bool(bool),
    Integer(i64),
    Float(f64),
    Function(FunctionPointer),
}

#[derive(Debug)]
pub enum RefData<'a> {
    String(&'a String),
    Function(&'a FunctionPointer),
}

impl<'a> RefData<'a> {
    pub fn get_function(&self) -> Result<&'a FunctionPointer, RuntimeError> {
        if let RefData::Function(pointer) = *self {
            return Ok(pointer);
        }
        Err(RuntimeError::TargetIsNotAFunction)
    }
}
