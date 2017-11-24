
use super::super::error::Error;
use super::super::builder::command::DataType;
use super::scope::Scope;

pub fn get_tuple_data_type(
    scope: &mut Scope,
    left_reg: usize,
    right_reg: usize,
) -> Result<(DataType, DataType), Error> {
    let left = scope.get_ref_holder(left_reg)?;
    let right = scope.get_ref_holder(right_reg)?;
    let left_b = left.borrow();
    let left = left_b.as_data_type();
    let right_b = right.borrow();
    let right = right_b.as_data_type();
    if left.is_none() || right.is_none() {
        return Err(Error::InvalidMathType);
    }
    Ok((left.unwrap(), right.unwrap()))
}
