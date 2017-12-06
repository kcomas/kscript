
use std::rc::Rc;
use std::cell::RefCell;
use super::super::error::Error;
use super::super::builder::command::DataType;
use super::scope::Scope;
use super::vm_types::DataContainer;

pub fn len(scope: &mut Scope, left_reg: usize, right_reg: usize) -> Result<(), Error> {
    scope.check_if_last(left_reg);
    let right = scope.get_ref_holder(right_reg)?;

    if !right.borrow().is_vector() {
        return Err(Error::InvalidArrayOpCall);
    }
    scope.set_value_in_reg(
        left_reg,
        DataContainer::Scalar(DataType::Integer(right.borrow().len() as i64)),
    );

    Ok(())
}

pub fn push(scope: &mut Scope, left_reg: usize, right_reg: usize) -> Result<(), Error> {
    let left = scope.get_ref_holder(left_reg)?;

    if let Some(ref mut array) = left.borrow_mut().underlying_array_mut() {
        let right = scope.get_ref_holder(right_reg)?;
        array.push(Rc::new(RefCell::new(right.borrow().reference_or_clone())));
        return Ok(());
    }
    Err(Error::InvalidArrayOpCall)
}
