use super::data_type::DataType;
use super::error::RuntimeError;

pub fn get_item_in_collection(
    accessor: &DataType,
    target: &DataType,
) -> Result<DataType, RuntimeError> {
    if accessor.is_int() {
        let index = accessor.as_int() as usize;
        if index >= target.len() {
            return Err(RuntimeError::IndexOutOfBound(
                target.clone(),
                accessor.clone(),
            ));
        }
        if target.is_string() {
            let string = target.as_string();
            let string = string.borrow();
            return Ok(DataType::Char(string.chars().nth(index).unwrap()));
        } else if target.is_array() {
            let array = target.get_array()?;
            let array = array.borrow();
            return Ok(array[index].clone());
        } else {
            return Err(RuntimeError::CannotAccessWithAccessor(
                target.clone(),
                accessor.clone(),
            ));
        }
    } else {
        return Err(RuntimeError::InvalidAccessor(accessor.clone()));
    }
}

pub fn update_elememnt_in_collection(
    accessor: &DataType,
    target: &DataType,
    value: DataType,
) -> Result<(), RuntimeError> {
    if accessor.is_int() {
        let index = accessor.as_int() as usize;
        if index >= target.len() {
            return Err(RuntimeError::IndexOutOfBound(
                target.clone(),
                accessor.clone(),
            ));
        }
        if target.is_string() {
            if !value.is_char() {
                return Err(RuntimeError::CannotInsertToString(value));
            }
            let string = target.as_string();
            let mut string = string.borrow_mut();
            string.remove(index);
            string.insert(index, value.as_char());
            return Ok(());
        } else if target.is_array() {
            let array = target.get_array()?;
            let mut array = array.borrow_mut();
            array[index] = value;
            return Ok(());
        } else {
            return Err(RuntimeError::CannotAccessWithAccessor(
                target.clone(),
                accessor.clone(),
            ));
        }
    } else {
        return Err(RuntimeError::InvalidAccessor(accessor.clone()));
    }
}
