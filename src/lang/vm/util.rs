
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use super::super::error::Error;
use super::super::builder::command::{DataType, DataHolder, Comparison};
use super::scope::Scope;
use super::vm_types::{DataContainer, RefMap, RefArray};

pub fn get_tuple_data_type(
    scope: &mut Scope,
    left_reg: usize,
    right_reg: usize,
) -> Result<(DataType, DataType), Error> {
    let left_r = scope.get_ref_holder(left_reg)?;
    let right_r = scope.get_ref_holder(right_reg)?;
    let left_b = left_r.borrow();
    let left = left_b.get_as_data_type();
    let right_b = right_r.borrow();
    let right = right_b.get_as_data_type();
    if left.is_none() || right.is_none() {
        return Err(Error::InvalidMathType);
    }
    Ok((left.unwrap(), right.unwrap()))
}


pub fn holder_deep_copy_conversion(
    scope: &Scope,
    data_holder: &DataHolder,
) -> Result<DataContainer, Error> {
    match *data_holder {
        DataHolder::Var(ref name) => {
            match scope.get_var(name) {
                Some(ref_holder) => Ok(ref_holder.borrow().clone()),
                None => Err(Error::VarNotDeclared),
            }
        }
        DataHolder::Const(ref name) => {
            match scope.get_const(name) {
                Some(ref_holder) => Ok(ref_holder.borrow().clone()),
                None => Err(Error::ConstNotDeclard),
            }
        }
        DataHolder::Anon(ref data_type) => Ok(DataContainer::Scalar(data_type.clone())),
        DataHolder::Array(ref data_holders) => {
            let mut containers: RefArray = Vec::new();
            for item in data_holders.iter() {
                containers.push(Rc::new(
                    RefCell::new(holder_deep_copy_conversion(scope, item)?),
                ));
            }
            Ok(DataContainer::Vector(containers))
        }
        DataHolder::Dict(ref dict) => {
            let mut hash_map: RefMap = HashMap::new();
            for (key, value) in dict {
                hash_map.insert(
                    key.clone(),
                    Rc::new(RefCell::new(holder_deep_copy_conversion(scope, value)?)),
                );
            }
            Ok(DataContainer::Hash(hash_map))
        }
        DataHolder::Math(reg) => {
            match scope.get_register(reg) {
                Some(reg_item) => {
                    match reg_item.to_ref_holder() {
                        Some(ref_holder) => Ok(ref_holder.borrow().clone()),
                        None => Err(Error::InvalidMathAccess),
                    }
                }
                None => Err(Error::InvalidMathAccess),
            }
        }
        DataHolder::Conditional(ref left_data, ref comp, ref right_data) => {
            let b = scope.evaluate_conditional(left_data, comp, right_data)?;
            Ok(DataContainer::Scalar(DataType::Bool(b)))
        }
        _ => Err(Error::CannotDeepCopyType),
    }
}

pub fn conditional_to_parts(
    conditional: &DataHolder,
) -> Result<(&DataHolder, &Comparison, &DataHolder), Error> {
    match *conditional {
        DataHolder::Conditional(ref left, ref cond, ref right) => Ok((left, cond, right)),
        _ => Err(Error::InvalidCondititonalHolder),
    }
}
