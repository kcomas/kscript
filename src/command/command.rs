use super::super::data_type::SharedDataType;

#[derive(Debug)]
pub enum Command {
    AddLocals,
    RemoveLocals,
    // add to stack
    Push(SharedDataType),
    // remove from stack
    // Pop,
    LoadStack(usize),
    LoadLocal(usize),
    SaveStack(usize),
    SaveLocal(usize),
    // create an array with values from stack
    MakeArray(usize),
    // pop 1 get accessor pop 2 get target
    Access,
    Equals,
    Add,
    Sub,
    Mul,
    Exp,
    Div,
    Rem,
    IoWrite,
    IoAppend,
    // jump to position if false
    Jmpf(usize),
    // number of args, function position
    Call(usize, usize),
    Return,
    // exit code
    Halt(i32),
}

impl Command {
    pub fn is_return(&self) -> bool {
        if let Command::Return = *self {
            return true;
        }
        false
    }

    pub fn is_halt(&self) -> bool {
        if let Command::Halt(_) = *self {
            return true;
        }
        false
    }
}
