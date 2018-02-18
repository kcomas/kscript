use super::function::FunctionPointer;

#[derive(Debug)]
pub enum Data {
    Bool(bool),
    Integer(i64),
    Float(f64),
    Function(FunctionPointer),
}

#[derive(Debug)]
pub enum RefData<'a> {
    Function(&'a FunctionPointer),
}
