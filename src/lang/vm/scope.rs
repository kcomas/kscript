
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use super::super::controller::Controller;
use super::super::logger::Logger;
use super::super::error::Error;
use super::super::builder::command::{DataHolder, DataType, CastTo, Comparison, coerce_numbers};
use super::util::{get_tuple_data_type, holder_deep_copy_conversion, holder_to_function_args,
                  run_function, access_object};
use super::vm_types::{RefHolder, RefMap, RefArray, DataContainer};

#[derive(Debug)]
pub enum RegItem {
    Var(RefHolder),
    Const(RefHolder),
    Access(RefHolder),
    Value(RefHolder),
    Empty,
}

impl RegItem {
    pub fn to_ref_holder(&self) -> Option<RefHolder> {
        match *self {
            RegItem::Var(ref ref_holder) |
            RegItem::Const(ref ref_holder) |
            RegItem::Access(ref ref_holder) |
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

    pub fn set_var(&mut self, name: &str, data: RefHolder) {
        self.vars.insert(name.to_string(), data);
    }

    pub fn get_const(&self, name: &str) -> Option<RefHolder> {
        match self.consts.get(name) {
            Some(ref_holder) => Some(ref_holder.clone()),
            None => None,
        }
    }

    pub fn set_const(&mut self, name: &str, data: RefHolder) {
        self.consts.insert(name.to_string(), data);
    }

    pub fn get_register(&self, reg: usize) -> Option<&RegItem> {
        match self.registers.get(reg) {
            Some(reg_item) => Some(reg_item),
            None => None,
        }
    }

    pub fn get_last_register_value(&self) -> DataContainer {
        match self.registers.last() {
            Some(ref reg_item) => {
                match reg_item.to_ref_holder() {
                    Some(ref_holder) => ref_holder.borrow().clone(),
                    None => DataContainer::Scalar(DataType::Null),
                }
            }
            None => DataContainer::Scalar(DataType::Null),
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

    pub fn can_sink(&self, reg: usize, allow_null_const: bool) -> Result<(), Error> {
        match self.registers.get(reg) {
            Some(target) => {
                match *target {
                    RegItem::Var(_) |
                    RegItem::Access(_) => Ok(()),
                    RegItem::Const(ref ref_holder) => {
                        if !allow_null_const {
                            return Err(Error::InvalidReferenceSet);
                        }
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

    pub fn evaluate_conditional<T: Logger>(
        &mut self,
        controller: &mut Controller<T>,
        left_data: &DataHolder,
        comp: &Comparison,
        right_data: &DataHolder,
    ) -> Result<bool, Error> {
        let left = holder_deep_copy_conversion(controller, self, left_data)?;
        let right = holder_deep_copy_conversion(controller, self, right_data)?;
        if !left.is_scalar() || !right.is_scalar() {
            return Err(Error::CanOnlyCompareScalars);
        }

        match *comp {
            Comparison::Equals => {
                Ok(
                    left.get_as_data_type_ref().unwrap() == right.get_as_data_type_ref().unwrap(),
                )
            }
            Comparison::NotEquals => {
                Ok(
                    left.get_as_data_type_ref().unwrap() != right.get_as_data_type_ref().unwrap(),
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

    pub fn set_register<T: Logger>(
        &mut self,
        controller: &mut Controller<T>,
        reg: usize,
        data_holder: &DataHolder,
    ) -> Result<(), Error> {
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
                    array_container.push(Rc::new(RefCell::new(
                        holder_deep_copy_conversion(controller, self, item)?,
                    )));
                }
                self.set_value_in_reg(reg, DataContainer::Vector(array_container));
            }
            DataHolder::Dict(ref dict) => {
                let mut hash_map: RefMap = HashMap::new();
                for (key, value) in dict {
                    hash_map.insert(
                        key.clone(),
                        Rc::new(RefCell::new(
                            holder_deep_copy_conversion(controller, self, value)?,
                        )),
                    );
                }
                self.set_value_in_reg(reg, DataContainer::Hash(hash_map));
            }
            DataHolder::ObjectAccess(ref target, ref accessor) => {
                self.registers[reg] =
                    RegItem::Access(access_object(controller, self, target, accessor)?);
            }
            DataHolder::Math(math_reg) => {
                self.set_value_in_reg(reg, DataContainer::Math(math_reg));
            }
            DataHolder::Conditional(ref left_data, ref comp, ref right_data) => {
                let b = self.evaluate_conditional(
                    controller,
                    left_data,
                    comp,
                    right_data,
                )?;
                self.set_value_in_reg(reg, DataContainer::Scalar(DataType::Bool(b)));
            }
            DataHolder::Function(ref data_holder_args, ref commands) => {
                self.set_value_in_reg(
                    reg,
                    DataContainer::Function(
                        holder_to_function_args(data_holder_args)?,
                        commands.clone(),
                    ),
                );
            }
            DataHolder::FunctionCall(ref target, ref args) => {
                let rst = run_function(controller, self, target, args)?;
                self.set_value_in_reg(reg, rst);
            }
            _ => return Err(Error::InvalidScopeRegisterSet),
        };
        Ok(())
    }

    pub fn clear_registers(&mut self) {
        self.registers.clear();
    }

    pub fn assign(&mut self, left_reg: usize, right_reg: usize) -> Result<(), Error> {
        let _ = self.can_sink(left_reg, true)?;
        let left = self.get_ref_holder(left_reg)?;
        let right = self.get_ref_holder(right_reg)?;
        *left.borrow_mut() = right.borrow().clone();
        Ok(())
    }

    pub fn take_reference(&mut self, left_reg: usize, right_reg: usize) -> Result<(), Error> {
        let _ = self.can_sink(left_reg, false)?;
        let left = self.get_ref_holder(left_reg)?;
        match self.registers.get(right_reg) {
            Some(ref target) => {
                match **target {
                    RegItem::Var(ref ref_holder) |
                    RegItem::Access(ref ref_holder) => {
                        *left.borrow_mut() = DataContainer::Reference(ref_holder.clone());
                        Ok(())
                    }
                    _ => Err(Error::InvalidReferenceGet),
                }
            }
            None => Err(Error::InvalidReferenceGet),
        }
    }

    pub fn cast(
        &mut self,
        cast_to: &CastTo,
        left_reg: usize,
        right_reg: usize,
    ) -> Result<(), Error> {
        self.check_if_last(left_reg);
        let right = self.get_ref_holder(right_reg)?;
        if let Some(data_type) = right.borrow().get_as_data_type_ref() {
            match *cast_to {
                CastTo::Integer => {
                    self.set_value_in_reg(
                        left_reg,
                        DataContainer::Scalar(DataType::Integer(data_type.cast_int()?)),
                    );
                }
                CastTo::Float => {
                    self.set_value_in_reg(
                        left_reg,
                        DataContainer::Scalar(DataType::Float(data_type.cast_float()?)),
                    );
                }
                CastTo::String => {
                    self.set_value_in_reg(
                        left_reg,
                        DataContainer::Scalar(DataType::String(data_type.cast_string()?)),
                    );
                }
                CastTo::File => return Err(Error::NYI),
                CastTo::Bool => {
                    self.set_value_in_reg(
                        left_reg,
                        DataContainer::Scalar(DataType::Bool(data_type.cast_bool()?)),
                    );
                }
            };
            return Ok(());
        }
        Err(Error::CastFail)
    }

    pub fn len(&mut self, left_reg: usize, right_reg: usize) -> Result<(), Error> {
        self.check_if_last(left_reg);
        let right = self.get_ref_holder(right_reg)?;

        if !right.borrow().is_vector() {
            return Err(Error::InvalidArrayOpCall);
        }
        self.set_value_in_reg(
            left_reg,
            DataContainer::Scalar(DataType::Integer(right.borrow().len() as i64)),
        );
        Ok(())
    }

    pub fn dereference(&mut self, left_reg: usize, right_reg: usize) -> Result<(), Error> {
        self.check_if_last(left_reg);
        let right = self.get_ref_holder(right_reg)?;
        let container = right.borrow();
        if container.is_reference() {
            if let Some(sub_container) = container.underlying_reference() {
                self.registers[left_reg] = RegItem::Var(sub_container);
                return Ok(());
            }
            return Err(Error::InvalidDereference);
        }
        Err(Error::InvalidDereference)
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
