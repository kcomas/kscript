use super::command::Command;
use super::address::MemoryAddress;

#[derive(Debug)]
pub struct Call {
    pub current_command_index: usize,
    pub entry_index: usize,
    pub number_arguments: usize,
    pub number_locals: usize,
}

#[derive(Debug)]
pub struct Vm {
    stack: Vec<MemoryAddress>,
    call_stack: Vec<Call>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            stack: Vec::new(),
            call_stack: Vec::new(),
        }
    }
}
