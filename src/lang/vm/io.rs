
use super::super::error::Error;
use super::super::builder::command::DataType;
use super::scope::Scope;

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
            }
            _ => return Err(Error::InvalidIoSink),
        }
        return Ok(());
    }
    Err(Error::InvalidIoSink)
}
