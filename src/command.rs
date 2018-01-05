use std::rc::Rc;
use super::data_type::DataType;

pub type SharedCommands = Rc<Vec<Command>>;

#[derive(Debug, Clone)]
pub enum Command {
    // Add values to the stack
    PushStack(DataType),
    // Comparisons
    // Run function
    Call,
    // Exit from function
    Return,
    // stop program with exit code
    Halt(i32),
}
