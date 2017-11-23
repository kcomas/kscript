
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use super::super::error::Error;
use super::super::builder::command::{DataHolder, DataType, Command};

pub type RefHolder = Rc<RefCell<DataType>>;

#[derive(Debug)]
pub enum RegItem {
    Data(RefHolder),
    Value(DataType),
}

#[derive(Debug)]
pub struct Scope {
    vars: HashMap<String, RefHolder>,
    consts: HashMap<String, RefHolder>,
    // cached files
    files: HashMap<String, Vec<Command>>,
    registers: Vec<RegItem>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            vars: HashMap::new(),
            consts: HashMap::new(),
            files: HashMap::new(),
            registers: Vec::new(),
        }
    }

    pub fn check_and_add_var(&mut self, name: &String) {
        self.registers.push(RegItem::Data(
            self.vars
                .entry(name.clone())
                .or_insert(Rc::new(RefCell::new(DataType::Null)))
                .clone(),
        ));
    }

    pub fn set_register(&mut self, reg: &usize, data_holder: &DataHolder) -> Result<(), Error> {
        if *reg == self.registers.len() {
            match *data_holder {
                DataHolder::Var(ref name) => self.check_and_add_var(name),
                DataHolder::Anon(ref data) => self.registers.push(RegItem::Value(data.clone())),
                _ => return Err(Error::InvalidScopeRegisterSet),
            };
            return Ok(());
        }
        Err(Error::InvalidScopeRegisterSet)
    }
}
