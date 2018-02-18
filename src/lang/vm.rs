use super::command::Command;
use super::address::MemoryAddress;
use super::memory::Memory;
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

    pub fn run(
        &mut self,
        memory: &mut Memory,
        commands: &Vec<Command>,
    ) -> Result<i32, RuntimeError> {
        loop {
            let command = {
                let current_call = self.get_current_call()?;
                match commands.get(current_call.current_command_index) {
                    Some(command) => command,
                    None => return Err(RuntimeError::InvalidCommandIndex),
                }
            };

            if let Some(exit_code) = self.match_command(memory, command)? {
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

    fn pop_stack(&mut self) -> Result<MemoryAddress, RuntimeError> {
        if let Some(item) = self.stack.pop() {
            return Ok(item);
        }
        Err(RuntimeError::CannotPopStack)
    }

    fn match_command(
        &mut self,
        memory: &mut Memory,
        command: &Command,
    ) -> Result<Option<i32>, RuntimeError> {
        match *command {
            Command::Push(ref address) => self.stack.push(address.clone()),
            Command::Add => {
                let right = self.pop_stack()?;
                let left = self.pop_stack()?;

                self.stack.push(memory.insert_counted(&left + &right));

                memory.dec(&right)?;
                memory.dec(&left)?;
            }
            Command::Sub => {
                let right = self.pop_stack()?;
                let left = self.pop_stack()?;

                self.stack.push(memory.insert_counted(&left - &right));

                memory.dec(&right)?;
                memory.dec(&left)?;
            }
            Command::Halt(exit_code) => return Ok(Some(exit_code)),
        }

        self.update_current_call_index(1);
        Ok(None)
    }
}
