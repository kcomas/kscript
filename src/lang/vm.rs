use super::memory::MemoryAddress;
use super::function::FunctionPointer;
use super::memory::Memory;
use super::data::DataHolder;
use super::command::Command;
use super::error::RuntimeError;

#[derive(Debug)]
pub struct Vm {
    stack: Vec<MemoryAddress>,
    stack_function_index: usize,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            stack: Vec::new(),
            stack_function_index: 0,
        }
    }

    pub fn init(&mut self, memory: &mut Memory, current_command_index: usize, num_locals: usize) {
        self.stack
            .push(memory.insert_counted(DataHolder::Function(FunctionPointer {
                current_command_index: current_command_index,
                num_arguments: 0,
                num_locals: num_locals,
            })));
    }

    pub fn run(
        &mut self,
        memory: &mut Memory,
        commands: &Vec<Command>,
    ) -> Result<i32, RuntimeError> {
        loop {
            let current_call = self.get_current_function_pointer()?;
            let command = match commands.get(current_call.current_command_index) {
                Some(command) => command,
                None => return Err(RuntimeError::InvalidCommandIndex),
            };

            let mabe_exit = self.match_command(memory, &current_call, command)?;

            if let Some(exit_code) = mabe_exit {
                return Ok(exit_code);
            }
        }
    }

    fn get_current_function_pointer(&self) -> Result<FunctionPointer, RuntimeError> {
        if let Some(item) = self.stack.get(self.stack_function_index) {
            return Ok(item.get_function()?);
        }
        Err(RuntimeError::CannotLoadCurrentFunction)
    }

    fn uppdate_current_function_comamnd_index(
        &mut self,
        new_command_index: usize,
    ) -> Result<(), RuntimeError> {
        if let Some(item) = self.stack.get_mut(self.stack_function_index) {
            let function = item.get_function_mut()?;
            function.current_command_index = new_command_index;
            return Ok(());
        }
        Err(RuntimeError::CannotUpdateCurrentFunction)
    }

    fn match_command(
        &mut self,
        memory: &mut Memory,
        current_call: &FunctionPointer,
        command: &Command,
    ) -> Result<Option<i32>, RuntimeError> {
        match *command {
            Command::Push(ref address) => self.stack.push(address.clone()),
            Command::Halt(exit_code) => return Ok(Some(exit_code)),
        };

        self.uppdate_current_function_comamnd_index(current_call.current_command_index + 1)?;
        Ok(None)
    }
}
