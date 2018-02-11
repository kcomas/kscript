use std::rc::Rc;
use std::cell::RefCell;
use super::function::FunctionPointer;

#[derive(Debug)]
pub enum DataHolder {
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(Rc<RefCell<String>>),
    Array(Vec<Rc<RefCell<DataHolder>>>),
    Function(FunctionPointer),
}
