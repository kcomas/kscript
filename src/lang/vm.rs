use super::memory::MemoryAddress;
use super::function::FunctionPointer;
use super::command::Command;
use super::memory::Memory;
use super::error::RuntimeError;

#[derive(Debug)]
pub struct Vm {
    stack: Vec<MemoryAddress>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm { stack: Vec::new() }
    }

    pub fn create_call_stack() -> Vec<FunctionPointer> {
        vec![
            FunctionPointer {
                entry_command_index: 0,
                current_command_index: 0,
                num_arguments: 0,
                num_locals: 0,
                function_length: 0,
            },
        ]
    }

    pub fn run(
        &mut self,
        commands: &Vec<Command>,
        memory: &mut Memory,
        call_stack: &mut Vec<FunctionPointer>,
    ) -> Result<i32, RuntimeError> {
        loop {
            let current_call = match call_stack.last_mut() {
                Some(current_call) => current_call,
                None => return Err(RuntimeError::CallStackEmpty),
            };

            let command = match commands.get(current_call.current_command_index) {
                Some(command) => command,
                None => return Err(RuntimeError::InvalidCommandIndex),
            };

            let (mabe_exit, mabe_call, mabe_return) =
                self.match_command(command, memory, current_call)?;

            if let Some(exit_code) = mabe_exit {
                return Ok(exit_code);
            }
        }
    }

    pub fn match_command(
        &mut self,
        command: &Command,
        memory: &mut Memory,
        current_call: &mut FunctionPointer,
    ) -> Result<(Option<i32>, Option<FunctionPointer>, bool), RuntimeError> {
        match *command {
            Command::PushStack(ref address) => self.stack.push(address.clone()),
            Command::Halt(exit_code) => return Ok((Some(exit_code), None, false)),
        }
        current_call.current_command_index += 1;
        Ok((None, None, false))
    }
}
