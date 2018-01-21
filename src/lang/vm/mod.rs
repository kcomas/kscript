mod command;
mod data_type;
mod error;

use self::command::Command;
use self::data_type::SharedData;
use self::error::VmError;

#[derive(Debug)]
pub struct FunctionScope {
    pub arg_index: usize,
    pub return_index: usize,
}

#[derive(Debug)]
pub struct Vm {
    stack: Vec<SharedData>,
    function_scopes: Vec<FunctionScope>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            stack: Vec::new(),
            function_scopes: Vec::new(),
        }
    }

    pub fn run(&mut self, commands: &Vec<Command>, entry: usize) -> Result<i32, VmError> {}
}
