use super::memory::Function;

#[derive(Debug)]
pub enum RefDataHolder<'a> {
    Bool(&'a bool),
    Integer(&'a i64),
    Float(&'a f64),
    Function(&'a Function),
}

#[derive(Debug)]
pub enum DataHolder {
    Bool(bool),
    Integer(i64),
    Float(f64),
    Function(Function),
}
