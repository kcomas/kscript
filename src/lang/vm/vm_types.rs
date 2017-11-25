
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use super::super::builder::command::DataType;

pub type RefHolder = Rc<RefCell<DataContainer>>;

#[derive(Debug, Clone, PartialEq)]
pub enum DataContainer {
    Scalar(DataType),
    Vector(Vec<RefHolder>),
    Hash(HashMap<String, RefHolder>),
    Math(usize),
}

impl DataContainer {
    pub fn as_data_type(&self) -> Option<DataType> {
        match *self {
            DataContainer::Scalar(ref data_type) => Some(data_type.clone()),
            _ => None,
        }
    }

    pub fn as_data_type_ref(&self) -> Option<&DataType> {
        match *self {
            DataContainer::Scalar(ref data_type) => Some(data_type),
            _ => None,
        }
    }
}

impl fmt::Display for DataContainer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DataContainer::Scalar(ref data_type) => write!(f, "{}", data_type),
            DataContainer::Vector(ref containers) => {
                let mut output = String::new();
                for item in containers.iter() {
                    output.push_str(&format!("{}", item.borrow()));
                }
                write!(f, "{}", output)
            }
            _ => write!(f, "{:?}", self),
        }
    }
}
