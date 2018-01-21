use std::rc::Rc;
use std::cell::RefCell;

pub type SharedData = Rc<RefCell<DataType>>;

#[derive(Debug)]
pub enum DataType {
    Bool(bool),
    Integer(i64),
    Float(f64),
    Array(Vec<SharedData>),
    Function(usize),
}
