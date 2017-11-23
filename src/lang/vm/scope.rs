
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use super::super::error::Error;
use super::super::builder::command::{DataHolder, DataType, Command};

pub type RefHolder = Rc<RefCell<DataType>>;

#[derive(Debug)]
pub enum RegItem {
    Var(RefHolder),
    Const(RefHolder),
    Value(RefHolder),
}

impl RegItem {
    pub fn to_ref_holder(&self) -> RefHolder {
        match *self {
            RegItem::Var(ref ref_holder) |
            RegItem::Const(ref ref_holder) |
            RegItem::Value(ref ref_holder) => ref_holder.clone(),
        }
    }
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
        self.registers.push(RegItem::Var(
            self.vars
                .entry(name.clone())
                .or_insert(Rc::new(RefCell::new(DataType::Null)))
                .clone(),
        ));
    }

    pub fn check_and_add_const(&mut self, name: &String) {
        self.registers.push(RegItem::Const(
            self.vars
                .entry(name.clone())
                .or_insert(Rc::new(RefCell::new(DataType::Null)))
                .clone(),
        ));
    }

    pub fn can_sink(&self, reg: usize) -> Result<(), Error> {
        match self.registers.get(reg) {
            Some(target) => {
                match *target {
                    RegItem::Var(_) => Ok(()),
                    _ => Err(Error::InvalidScopeSink),
                }
            }
            None => Err(Error::InvalidScopeRegisterGet),
        }
    }

    pub fn get_ref_holder(&self, reg: usize) -> Result<RefHolder, Error> {
        match self.registers.get(reg) {
            Some(reg_item) => Ok(reg_item.to_ref_holder()),
            None => Err(Error::InvalidScopeRegisterGet),
        }
    }

    pub fn set_register(&mut self, reg: usize, data_holder: &DataHolder) -> Result<(), Error> {
        if reg == self.registers.len() {
            match *data_holder {
                DataHolder::Var(ref name) => self.check_and_add_var(name),
                DataHolder::Const(ref name) => self.check_and_add_const(name),
                DataHolder::Anon(ref data) => {
                    self.registers.push(RegItem::Value(
                        Rc::new(RefCell::new(data.clone())),
                    ))
                }
                _ => return Err(Error::InvalidScopeRegisterSet),
            };
            return Ok(());
        }
        Err(Error::InvalidScopeRegisterSet)
    }

    pub fn assign(&mut self, left_reg: usize, right_reg: usize) -> Result<(), Error> {
        let _ = self.can_sink(left_reg)?;
        let left = self.get_ref_holder(left_reg)?;
        let right = self.get_ref_holder(right_reg)?;
        *left.borrow_mut() = right.borrow().clone();
        Ok(())
    }

    pub fn clear_registers(&mut self) {
        self.registers.clear();
    }
}
