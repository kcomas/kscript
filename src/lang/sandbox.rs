
use super::error::{Error, SandBoxError};
use super::builder::command::{DataHolder, DataType};
use super::vm::scope::Scope;

pub trait IoSandBox {
    fn new() -> Self;

    fn io_write(&self, content: &DataHolder) -> Result<(), Error> {
        Err(Error::SandBoxLock(SandBoxError::Io))
    }

    fn io_append(&self, content: &DataHolder) -> Result<(), Error> {
        Err(Error::SandBoxLock(SandBoxError::Io))
    }
}
