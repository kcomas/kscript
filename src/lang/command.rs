use std::rc::Rc;
use super::data_type::DataType;

pub type SharedCommands = Rc<Vec<Command>>;

#[derive(Debug, Clone)]
pub enum Command {
    // Add values to the stack
    PushStack(DataType),
    // Locals
    SaveLocal(usize),
    LoadLocal(usize),
    // Comparisons
    Equals,
    // Math
    Add,
    Sub,
    Mul,
    // Joins
    Concat,
    // Jumps all relative
    // if the top boolean in the stack is false
    JumpIfFalse(usize),
    // Run function
    Call,
    CallSelf,
    // load an argument from the stack at an offset from the current function stack index
    SaveStackArg(usize),
    LoadStackArg(usize),
    // Exit from function
    Return,
    // IO
    IoAppend,
    // stop program with exit code
    Halt(i32),
}

impl Command {
    pub fn is_return(&self) -> bool {
        if let Command::Return = *self {
            return true;
        }
        false
    }
}
