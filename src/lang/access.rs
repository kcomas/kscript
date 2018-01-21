use super::data_type::DataType;
use super::error::RuntimeError;

pub fn get_item_in_collection(
    accessor: DataType,
    target: DataType,
) -> Result<DataType, RuntimeError> {
    if accessor.is_int() {
        let index = accessor.as_int() as usize;
        if target.is_string() {
            let string = target.get_string()?;
            let string = string.borrow();
            if let Some(c) = string.chars().nth(index) {
                return Ok(DataType::Char(c));
            }
        } else if target.is_array() {
            let array = target.get_array()?;
            let array = array.borrow();
            if let Some(item) = array.get(index) {
                return Ok(item.clone());
            }
        }
        return Err(RuntimeError::CannotAccessWithAccessor);
    }
    Err(RuntimeError::InvalidAccessor)
}

pub fn update_elememnt_in_collection(
    accessor: DataType,
    target: DataType,
    value: DataType,
) -> Result<(), RuntimeError> {
    if accessor.is_int() {
        let index = accessor.as_int() as usize;
        if target.is_string() {
            if !value.is_char() {
                return Err(RuntimeError::CannotInsertToString);
            }
            let string = target.get_string()?;
            let mut string = string.borrow_mut();
            string.remove(index);
            string.insert(index, value.as_char());
            return Ok(());
        } else if target.is_array() {
            let array = target.get_array()?;
            let mut array = array.borrow_mut();
            if let Some(item) = array.get_mut(index) {
                *item = value;
                return Ok(());
            }
        }
        return Err(RuntimeError::CannotAccessWithAccessor);
    }
    Err(RuntimeError::InvalidAccessor)
}
