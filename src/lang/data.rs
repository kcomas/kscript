use super::function::FunctionPointer;

pub enum DataHolder {
    Bool(bool),
    Integer(i64),
    Float(f64),
    Function(FunctionPointer),
}

pub enum RefHolder<'a> {
    Bool(&'a bool),
    Integer(&'a i64),
    Float(&'a f64),
    Function(&'a FunctionPointer),
}
