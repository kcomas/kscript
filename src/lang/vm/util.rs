
use super::super::error::Error;
use super::super::builder::command::{DataType, DataHolder};
use super::scope::Scope;

pub fn get_tuple_data_type(
    scope: &mut Scope,
    left_reg: usize,
    right_reg: usize,
) -> Result<(DataType, DataType), Error> {
    let left_r = scope.get_ref_holder(left_reg)?;
    let right_r = scope.get_ref_holder(right_reg)?;
    let left_b = left_r.borrow();
    let left = left_b.as_data_type();
    let right_b = right_r.borrow();
    let right = right_b.as_data_type();
    if left.is_none() || right.is_none() {
        return Err(Error::InvalidMathType);
    }
    Ok((left.unwrap(), right.unwrap()))
}


// pub fn holder_deep_copy(scope: &mut Scope, data_holder: &DataHolder) -> DataHolder {
//     match *data_holder {
//
//     }
// }
