
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use super::super::builder::command::DataType;

pub type RefHolder = Rc<RefCell<DataContainer>>;
pub type RefMap = HashMap<String, RefHolder>;
pub type RefArray = Vec<RefHolder>;

#[derive(Debug, PartialEq)]
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

    pub fn is_scalar(&self) -> bool {
        match *self {
            DataContainer::Scalar(_) => true,
            _ => false,
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

impl Clone for DataContainer {
    fn clone(&self) -> DataContainer {
        match *self {
            DataContainer::Scalar(ref data_type) => DataContainer::Scalar(data_type.clone()),
            DataContainer::Vector(ref array) => {
                let mut new_rcs: RefArray = Vec::new();
                for rc in array {
                    new_rcs.push(Rc::new(RefCell::new(rc.borrow().clone())));
                }
                DataContainer::Vector(new_rcs)
            }
            DataContainer::Hash(ref hash) => {
                let mut new_hash: RefMap = HashMap::new();
                for (key, value) in hash {
                    new_hash.insert(key.clone(), Rc::new(RefCell::new(value.borrow().clone())));
                }
                DataContainer::Hash(new_hash)
            }
            DataContainer::Math(size) => DataContainer::Math(size),
        }
    }
}