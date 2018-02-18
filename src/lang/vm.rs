use super::command::Command;
use super::address::MemoryAddress;
use super::error::RuntimeError;

#[derive(Debug)]
pub struct Call {
    pub current_command_index: usize,
    pub entry_index: usize,
    pub stack_index: usize,
    pub number_arguments: usize,
    pub number_locals: usize,
}

#[derive(Debug)]
pub struct Vm {
    stack: Vec<MemoryAddress>,
    call_stack: Vec<Call>,
}

impl Vm {
    pub fn new(entry: usize) -> Vm {
        Vm {
            stack: Vec::new(),
            call_stack: vec![
                Call {
                    current_command_index: entry,
                    entry_index: entry,
                    stack_index: 0,
                    number_arguments: 0,
                    number_locals: 0,
                },
            ],
        }
    }

    pub fn run(&mut self, commands: &Vec<Command>) -> Result<i32, RuntimeError> {
        loop {
            let command = {
                let current_call = self.get_current_call()?;
                match commands.get(current_call.current_command_index) {
                    Some(command) => command,
                    None => return Err(RuntimeError::InvalidCommandIndex),
                }
            };

            if let Some(exit_code) = self.match_command(command)? {
                return Ok(exit_code);
            }
        }
    }

    fn get_current_call(&self) -> Result<&Call, RuntimeError> {
        if let Some(call) = self.call_stack.last() {
            return Ok(call);
        }
        Err(RuntimeError::CannotLoadCurrentCall)
    }

    fn update_current_call_index(&mut self, command_inc: usize) -> Result<(), RuntimeError> {
        if let Some(call) = self.call_stack.last_mut() {
            call.current_command_index += 1;
            return Ok(());
        }
        Err(RuntimeError::CannotUpdateCurrentCall)
    }

    fn match_command(&mut self, command: &Command) -> Result<Option<i32>, RuntimeError> {
        match *command {
            Command::Push(ref address) => self.stack.push(address.clone()),
            Command::Halt(exit_code) => return Ok(Some(exit_code)),
        }

        self.update_current_call_index(1);
        Ok(None)
    }
}
