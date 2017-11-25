
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use super::super::error::Error;
use super::super::builder::command::{DataHolder, DataType, Command, Comparison, coerce_numbers};
use super::util::{get_tuple_data_type, holder_deep_copy_conversion};
use super::vm_types::{RefHolder, RefMap, RefArray, DataContainer};


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
    id: usize,
    vars: RefMap,
    consts: RefMap,
    registers: Vec<RegItem>,
}

impl Scope {
    pub fn new(id: usize) -> Scope {
        Scope {
            id: id,
            vars: HashMap::new(),
            consts: HashMap::new(),
            registers: Vec::new(),
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
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

    pub fn get_register(&self, reg: usize) -> Option<&RegItem> {
        match self.registers.get(reg) {
            Some(reg_item) => Some(reg_item),
            None => None,
        }
    }

    fn check_and_add_var(&mut self, reg: usize, name: &String) {
        self.registers[reg] = RegItem::Var(
            self.vars
                .entry(name.clone())
                .or_insert(Rc::new(RefCell::new(DataContainer::Scalar(DataType::Null))))
                .clone(),
        );
    }

    fn check_and_add_const(&mut self, reg: usize, name: &String) {
        self.registers[reg] = RegItem::Const(
            self.consts
                .entry(name.clone())
                .or_insert(Rc::new(RefCell::new(DataContainer::Scalar(DataType::Null))))
                .clone(),
        );
    }

    fn can_sink(&self, reg: usize) -> Result<(), Error> {
        match self.registers.get(reg) {
            Some(target) => {
                match *target {
                    RegItem::Var(_) => Ok(()),
                    RegItem::Const(ref ref_holder) => {
                        match *ref_holder.borrow() {
                            DataContainer::Scalar(ref data) => {
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
                            DataContainer::Math(math_reg) => Ok(self.get_ref_holder(math_reg)?),
                            _ => Ok(reg_item.clone()),
                        }
                    }
                    None => Err(Error::InvalidScopeRegisterGet),
                }
            }
            None => Err(Error::InvalidScopeRegisterGet),
        }
    }

    fn check_if_last(&mut self, reg: usize) {
        if reg == self.registers.len() {
            self.registers.push(RegItem::Empty);
        }
    }

    fn set_value_in_reg(&mut self, sink_reg: usize, value: DataContainer) {
        self.registers[sink_reg] = RegItem::Value(Rc::new(RefCell::new(value)));
    }

    pub fn evaluate_conditional(
        &self,
        left_data: &DataHolder,
        comp: &Comparison,
        right_data: &DataHolder,
    ) -> Result<bool, Error> {
        let left = holder_deep_copy_conversion(self, left_data)?;
        let right = holder_deep_copy_conversion(self, right_data)?;
        if !left.is_scalar() || !right.is_scalar() {
            return Err(Error::CanOnlyCompareScalars);
        }

        match *comp {
            Comparison::Equals => {
                Ok(
                    left.get_as_data_type_ref().unwrap() == right.get_as_data_type_ref().unwrap(),
                )
            }
            Comparison::EqualOrGreater => {
                let (left, right) = coerce_numbers(
                    left.get_as_data_type_ref().unwrap(),
                    right.get_as_data_type_ref().unwrap(),
                );
                if left.is_int() && right.is_int() {
                    return Ok(left.get_as_int() >= right.get_as_int());
                }
                Ok(left.get_as_float() >= right.get_as_float())
            }
            Comparison::EqualOrLess => {
                let (left, right) = coerce_numbers(
                    left.get_as_data_type_ref().unwrap(),
                    right.get_as_data_type_ref().unwrap(),
                );
                if left.is_int() && right.is_int() {
                    return Ok(left.get_as_int() <= right.get_as_int());
                }
                Ok(left.get_as_float() <= right.get_as_float())
            }
            Comparison::Greater => {
                let (left, right) = coerce_numbers(
                    left.get_as_data_type_ref().unwrap(),
                    right.get_as_data_type_ref().unwrap(),
                );
                if left.is_int() && right.is_int() {
                    return Ok(left.get_as_int() > right.get_as_int());
                }
                Ok(left.get_as_float() > right.get_as_float())
            }
            Comparison::Less => {
                let (left, right) = coerce_numbers(
                    left.get_as_data_type_ref().unwrap(),
                    right.get_as_data_type_ref().unwrap(),
                );
                if left.is_int() && right.is_int() {
                    return Ok(left.get_as_int() < right.get_as_int());
                }
                Ok(left.get_as_float() < right.get_as_float())
            }
            Comparison::And => {
                Ok(
                    left.get_as_data_type_ref().unwrap().get_as_bool() &&
                        right.get_as_data_type_ref().unwrap().get_as_bool(),
                )
            }
            Comparison::Or => {
                Ok(
                    left.get_as_data_type_ref().unwrap().get_as_bool() ||
                        right.get_as_data_type_ref().unwrap().get_as_bool(),
                )
            }
        }
    }

    pub fn set_register(&mut self, reg: usize, data_holder: &DataHolder) -> Result<(), Error> {
        self.check_if_last(reg);
        match *data_holder {
            DataHolder::Var(ref name) => self.check_and_add_var(reg, name),
            DataHolder::Const(ref name) => self.check_and_add_const(reg, name),
            DataHolder::Anon(ref data) => {
                self.set_value_in_reg(reg, DataContainer::Scalar(data.clone()));
            }
            DataHolder::Array(ref holders) => {
                let mut array_container: RefArray = Vec::new();
                for item in holders {
                    array_container.push(Rc::new(
                        RefCell::new(holder_deep_copy_conversion(self, item)?),
                    ));
                }
                self.set_value_in_reg(reg, DataContainer::Vector(array_container));
            }
            DataHolder::Dict(ref dict) => {
                let mut hash_map: RefMap = HashMap::new();
                for (key, value) in dict {
                    hash_map.insert(
                        key.clone(),
                        Rc::new(RefCell::new(holder_deep_copy_conversion(self, value)?)),
                    );
                }
                self.set_value_in_reg(reg, DataContainer::Hash(hash_map));
            }
            DataHolder::Math(math_reg) => {
                self.set_value_in_reg(reg, DataContainer::Math(math_reg));
            }
            DataHolder::Conditional(ref left_data, ref comp, ref right_data) => {
                let b = self.evaluate_conditional(left_data, comp, right_data)?;
                self.set_value_in_reg(reg, DataContainer::Scalar(DataType::Bool(b)));
            }
            _ => return Err(Error::InvalidScopeRegisterSet),
        };
        Ok(())
    }

    pub fn clear_registers(&mut self) {
        self.registers.clear();
    }

    pub fn assign(&mut self, left_reg: usize, right_reg: usize) -> Result<(), Error> {
        let _ = self.can_sink(left_reg)?;
        let left = self.get_ref_holder(left_reg)?;
        let right = self.get_ref_holder(right_reg)?;
        *left.borrow_mut() = right.borrow().clone();
        Ok(())
    }

    pub fn io_write(&mut self, left_reg: usize, right_reg: usize) -> Result<(), Error> {
        let left = self.get_ref_holder(left_reg)?;
        let right = self.get_ref_holder(right_reg)?;
        if let Some(data_holder) = right.borrow().get_as_data_type_ref() {
            match *data_holder {
                DataType::Integer(int) => {
                    match int {
                        1 => print!("{}", left.borrow()),
                        2 => eprint!("{}", left.borrow()),
                        _ => return Err(Error::InvalidIoSink),
                    }
                }
                _ => return Err(Error::InvalidIoSink),
            }
            return Ok(());
        }
        Err(Error::InvalidIoSink)
    }

    pub fn io_append(&mut self, left_reg: usize, right_reg: usize) -> Result<(), Error> {
        let left = self.get_ref_holder(left_reg)?;
        let right = self.get_ref_holder(right_reg)?;
        if let Some(data_holder) = right.borrow().get_as_data_type_ref() {
            match *data_holder {
                DataType::Integer(int) => {
                    match int {
                        1 => println!("{}", left.borrow()),
                        2 => eprintln!("{}", left.borrow()),
                        _ => return Err(Error::InvalidIoSink),
                    }
                }
                _ => return Err(Error::InvalidIoSink),
            }
            return Ok(());
        }
        Err(Error::InvalidIoSink)
    }

    pub fn addition(
        &mut self,
        sink_reg: usize,
        left_reg: usize,
        right_reg: usize,
    ) -> Result<(), Error> {
        self.check_if_last(sink_reg);
        let (left, right) = get_tuple_data_type(self, left_reg, right_reg)?;
        self.set_value_in_reg(sink_reg, DataContainer::Scalar(left + right));
        Ok(())
    }

    pub fn subtract(
        &mut self,
        sink_reg: usize,
        left_reg: usize,
        right_reg: usize,
    ) -> Result<(), Error> {
        self.check_if_last(sink_reg);
        let (left, right) = get_tuple_data_type(self, left_reg, right_reg)?;
        self.set_value_in_reg(sink_reg, DataContainer::Scalar(left - right));
        Ok(())
    }

    pub fn multiply(
        &mut self,
        sink_reg: usize,
        left_reg: usize,
        right_reg: usize,
    ) -> Result<(), Error> {
        self.check_if_last(sink_reg);
        let (left, right) = get_tuple_data_type(self, left_reg, right_reg)?;
        self.set_value_in_reg(sink_reg, DataContainer::Scalar(left * right));
        Ok(())
    }

    pub fn divide(
        &mut self,
        sink_reg: usize,
        left_reg: usize,
        right_reg: usize,
    ) -> Result<(), Error> {
        self.check_if_last(sink_reg);
        let (left, right) = get_tuple_data_type(self, left_reg, right_reg)?;
        self.set_value_in_reg(sink_reg, DataContainer::Scalar(left / right));
        Ok(())
    }

    pub fn modulus(
        &mut self,
        sink_reg: usize,
        left_reg: usize,
        right_reg: usize,
    ) -> Result<(), Error> {
        self.check_if_last(sink_reg);
        let (left, right) = get_tuple_data_type(self, left_reg, right_reg)?;
        self.set_value_in_reg(sink_reg, DataContainer::Scalar(left % right));
        Ok(())
    }

    pub fn exponent(
        &mut self,
        sink_reg: usize,
        left_reg: usize,
        right_reg: usize,
    ) -> Result<(), Error> {
        self.check_if_last(sink_reg);
        let (left, right) = get_tuple_data_type(self, left_reg, right_reg)?;
        if left.is_int() && right.is_int() {
            self.set_value_in_reg(
                sink_reg,
                DataContainer::Scalar(DataType::Integer(
                    left.get_as_int().pow(right.get_as_int() as u32),
                )),
            );
            return Ok(());
        } else if left.is_float() && right.is_int() {
            self.set_value_in_reg(
                sink_reg,
                DataContainer::Scalar(DataType::Float(
                    left.get_as_float().powi(right.get_as_int() as i32),
                )),
            );
        }
        self.set_value_in_reg(
            sink_reg,
            DataContainer::Scalar(DataType::Float(
                left.get_as_float().powf(right.get_as_float()),
            )),
        );
        Ok(())
    }
}
