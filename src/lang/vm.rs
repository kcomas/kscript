use super::command::Command;
use super::memory::{Memory, MemoryAddress};
use super::error::RuntimeError;

#[derive(Debug)]
pub struct CallInfo {
    pub function_memory_address: usize,
    pub function_command_index: usize,
}

#[derive(Debug)]
pub struct Vm {
    stack: Vec<MemoryAddress>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm { stack: Vec::new() }
    }

    pub fn create_calls(function_memory_address: usize) -> Vec<CallInfo> {
        vec![
            CallInfo {
                function_memory_address: function_memory_address,
                function_command_index: 0,
            },
        ]
    }

    pub fn run(
        &mut self,
        memory: &mut Memory,
        calls: &mut Vec<CallInfo>,
    ) -> Result<i32, RuntimeError> {
        loop {
            let (mabe_new_calls, do_return, mabe_exit_code) = match calls.last_mut() {
                Some(ref mut current_calls) => self.match_command(memory, current_calls)?,
                None => return Err(RuntimeError::CallsEmpty),
            };

            if let Some(exit_code) = mabe_exit_code {
                return Ok(exit_code);
            }
        }
    }

    pub fn pop_stack(&mut self) -> Result<MemoryAddress, RuntimeError> {
        if let Some(addr) = self.stack.pop() {
            return Ok(addr);
        }
        Err(RuntimeError::CannotPopStackEmpty)
    }

    pub fn match_command(
        &mut self,
        memory: &mut Memory,
        current_calls: &mut CallInfo,
    ) -> Result<(Option<CallInfo>, bool, Option<i32>), RuntimeError> {
        let command = match memory
            .get_function(current_calls.function_memory_address)
            .get_command(current_calls.function_command_index)
        {
            Some(command) => command,
            None => return Err(RuntimeError::InvalidCommandIndex),
        };

        match command {
            Command::PushStack(addr) => self.stack.push(addr),
            Command::Add => {
                let right = self.pop_stack()?;
                let left = self.pop_stack()?;
                let rst = memory.get(&left) + memory.get(&right);
                memory.dec(&left);
                memory.dec(&right);
                self.stack.push(memory.insert(rst));
            }
            Command::Sub => {
                let right = self.pop_stack()?;
                let left = self.pop_stack()?;
                let rst = memory.get(&left) - memory.get(&right);
                memory.dec(&left);
                memory.dec(&right);
                self.stack.push(memory.insert(rst));
            }
            Command::Halt(exit_code) => return Ok((None, false, Some(exit_code))),
        };

        current_calls.function_command_index += 1;

        Ok((None, false, None))
    }
}
