
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use super::super::error::Error;
use super::super::builder::command::{DataHolder, DataType, Command};

pub type RefHolder = Rc<RefCell<DataType>>;

#[derive(Debug)]
pub enum RegItem<'a> {
    Data(RefHolder),
    Value(&'a DataHolder),
}

pub struct Scope<'a> {
    vars: HashMap<String, RefHolder>,
    consts: HashMap<String, RefHolder>,
    // cached files
    files: HashMap<String, Vec<Command>>,
    registers: Vec<RegItem<'a>>,
}

impl<'a> Scope<'a> {
    pub fn new() -> Scope<'a> {
        Scope {
            vars: HashMap::new(),
            consts: HashMap::new(),
            files: HashMap::new(),
            registers: Vec::new(),
        }
    }

    pub fn check_and_push_var(&mut self, reg: &usize, name: &String) {
        self.vars.entry(name.clone()).or_insert(Rc::new(
            RefCell::new(DataType::Null),
        ));
    }

    pub fn set_register(&mut self, reg: &usize, data_holder: &'a DataHolder) -> Result<(), Error> {
        if *reg == self.registers.len() {
            match *data_holder {
                DataHolder::Var(ref name) => self.check_and_push_var(reg, name),
                _ => self.registers.push(RegItem::Value(data_holder)),
            }
            return Ok(());
        }
        Err(Error::InvalidScopeRegisterSet)
    }
}
