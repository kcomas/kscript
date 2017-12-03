
use std::io::{self, Write};
use super::super::error::Error;
use super::super::builder::command::DataType;
use super::scope::Scope;
use super::vm_types::DataContainer;

pub fn io_write(scope: &mut Scope, left_reg: usize, right_reg: usize) -> Result<(), Error> {
    let left = scope.get_ref_holder(left_reg)?;
    let right = scope.get_ref_holder(right_reg)?;
    if let Some(data_holder) = right.borrow().get_as_data_type_ref() {
        match *data_holder {
            DataType::Integer(int) => {
                match int {
                    1 => print!("{}", left.borrow()),
                    2 => eprint!("{}", left.borrow()),
                    _ => return Err(Error::InvalidIoSink),
                }
                let _ = io::stdout().flush().unwrap();
            }
            _ => return Err(Error::InvalidIoSink),
        }
        return Ok(());
    }
    Err(Error::InvalidIoSink)
}

pub fn io_append(scope: &mut Scope, left_reg: usize, right_reg: usize) -> Result<(), Error> {
    let left = scope.get_ref_holder(left_reg)?;
    let right = scope.get_ref_holder(right_reg)?;
    if let Some(data_holder) = right.borrow().get_as_data_type_ref() {
        match *data_holder {
            DataType::Integer(int) => {
                match int {
                    1 => println!("{}", left.borrow()),
                    2 => eprintln!("{}", left.borrow()),
                    _ => return Err(Error::InvalidIoSink),
                }
                let _ = io::stdout().flush().unwrap();
            }
            _ => return Err(Error::InvalidIoSink),
        }
        return Ok(());
    }
    Err(Error::InvalidIoSink)
}

pub fn io_read(scope: &mut Scope, left_reg: usize, right_reg: usize) -> Result<(), Error> {
    let left = scope.get_ref_holder(left_reg)?;
    let right = scope.get_ref_holder(right_reg)?;
    if let Some(data_holder) = right.borrow().get_as_data_type_ref() {
        match *data_holder {
            DataType::Integer(int) => {
                if int == 0 {
                    let mut input = String::new();
                    let _ = io::stdin().read_line(&mut input).unwrap();
                    let trimmed = input.trim_right().len();
                    input.truncate(trimmed);
                    *left.borrow_mut() = DataContainer::Scalar(DataType::String(input));
                } else {
                    return Err(Error::InvalidIoSource);
                }
            }
            _ => return Err(Error::InvalidIoSource),
        }
        return Ok(());
    }
    Err(Error::InvalidIoSource)
}

pub fn io_read_append(scope: &mut Scope, left_reg: usize, right_reg: usize) -> Result<(), Error> {
    let left = scope.get_ref_holder(left_reg)?;
    let mut left_mut_borrow = left.borrow_mut();
    let right = scope.get_ref_holder(right_reg)?;
    if let Some(data_holder) = right.borrow().get_as_data_type_ref() {
        match *data_holder {
            DataType::Integer(int) => {
                if int == 0 {
                    let mut input = String::new();
                    let _ = io::stdin().read_line(&mut input).unwrap();
                    let trimmed = input.trim_right().len();
                    input.truncate(trimmed);
                    let string = format!("{}{}", left_mut_borrow, input);
                    *left_mut_borrow = DataContainer::Scalar(DataType::String(string));
                } else {
                    return Err(Error::InvalidIoSource);
                }
            }
            _ => return Err(Error::InvalidIoSource),
        }
        return Ok(());
    }
    Err(Error::InvalidIoSource)
}
