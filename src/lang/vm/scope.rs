
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use super::super::error::Error;
use super::super::builder::command::{DataHolder, DataType, Command};
use super::util::get_tuple_data_type;

pub type RefHolder = Rc<RefCell<DataHolder>>;

#[derive(Debug)]
pub enum RegItem {
    Var(RefHolder),
    Const(RefHolder),
    Value(RefHolder),
    Empty,
}

impl RegItem {
    pub fn to_ref_holder(&self) -> Option<RefHolder> {
        match *self {
            RegItem::Var(ref ref_holder) |
            RegItem::Const(ref ref_holder) |
            RegItem::Value(ref ref_holder) => Some(ref_holder.clone()),
            _ => None,
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

    pub fn get_var(&self, name: &str) -> Option<RefHolder> {
        match self.vars.get(name) {
            Some(ref_holder) => Some(ref_holder.clone()),
            None => None,
        }
    }

    pub fn get_const(&self, name: &str) -> Option<RefHolder> {
        match self.consts.get(name) {
            Some(ref_holder) => Some(ref_holder.clone()),
            None => None,
        }
    }

    pub fn check_and_add_var(&mut self, reg: usize, name: &String) {
        self.registers[reg] = RegItem::Var(
            self.vars
                .entry(name.clone())
                .or_insert(Rc::new(RefCell::new(DataHolder::Anon(DataType::Null))))
                .clone(),
        );
    }

    pub fn check_and_add_const(&mut self, reg: usize, name: &String) {
        self.registers[reg] = RegItem::Const(
            self.consts
                .entry(name.clone())
                .or_insert(Rc::new(RefCell::new(DataHolder::Anon(DataType::Null))))
                .clone(),
        );
    }

    pub fn can_sink(&self, reg: usize) -> Result<(), Error> {
        match self.registers.get(reg) {
            Some(target) => {
                match *target {
                    RegItem::Var(_) => Ok(()),
                    RegItem::Const(ref ref_holder) => {
                        match *ref_holder.borrow() {
                            DataHolder::Anon(ref data) => {
                                match *data {
                                    DataType::Null => Ok(()),
                                    _ => Err(Error::InvalidScopeSink),
                                }
                            }
                            _ => Err(Error::InvalidScopeSink),
                        }
                    }
                    _ => Err(Error::InvalidScopeSink),
                }
            }
            None => Err(Error::InvalidScopeRegisterGet),
        }
    }

    pub fn get_ref_holder(&self, reg: usize) -> Result<RefHolder, Error> {
        match self.registers.get(reg) {
            Some(mabe_reg_item) => {
                match mabe_reg_item.to_ref_holder() {
                    Some(reg_item) => {
                        match *reg_item.borrow() {
                            DataHolder::Math(math_reg) => Ok(self.get_ref_holder(math_reg)?),
                            _ => Ok(reg_item.clone()),
                        }
                    }
                    None => Err(Error::InvalidScopeRegisterGet),
                }
            }
            None => Err(Error::InvalidScopeRegisterGet),
        }
    }

    pub fn check_if_last(&mut self, reg: usize) {
        if reg == self.registers.len() {
            self.registers.push(RegItem::Empty);
        }
    }

    pub fn set_register(&mut self, reg: usize, data_holder: &DataHolder) -> Result<(), Error> {
        self.check_if_last(reg);
        match *data_holder {
            DataHolder::Var(ref name) => self.check_and_add_var(reg, name),
            DataHolder::Const(ref name) => self.check_and_add_const(reg, name),
            DataHolder::Anon(ref data) => {
                self.registers[reg] =
                    RegItem::Value(Rc::new(RefCell::new(DataHolder::Anon(data.clone()))));
            }
            DataHolder::Math(math_reg) => {
                self.registers[reg] =
                    RegItem::Value(Rc::new(RefCell::new(DataHolder::Math(math_reg))));
            }
            _ => return Err(Error::InvalidScopeRegisterSet),
        };
        Ok(())
    }

    pub fn assign(&mut self, left_reg: usize, right_reg: usize) -> Result<(), Error> {
        let _ = self.can_sink(left_reg)?;
        let left = self.get_ref_holder(left_reg)?;
        let right = self.get_ref_holder(right_reg)?;
        *left.borrow_mut() = right.borrow().clone();
        Ok(())
    }

    pub fn addition(
        &mut self,
        sink_reg: usize,
        left_reg: usize,
        right_reg: usize,
    ) -> Result<(), Error> {
        self.check_if_last(sink_reg);
        let (left, right) = get_tuple_data_type(self, left_reg, right_reg)?;
        self.registers[sink_reg] =
            RegItem::Value(Rc::new(RefCell::new(DataHolder::Anon(left + right))));
        Ok(())
    }

    pub fn clear_registers(&mut self) {
        self.registers.clear();
    }
}
