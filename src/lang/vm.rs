use super::command::Command;
use super::memory::{Memory, MemoryAddress};
use super::error::RuntimeError;

#[derive(Debug)]
pub struct CallInfo {
    pub function_memory_address: usize,
    pub function_command_index: usize,
    pub num_arguments: usize,
    pub argument_stack_index: usize,
    pub locals: Vec<MemoryAddress>,
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
                num_arguments: 0,
                argument_stack_index: 0,
                locals: Vec::new(),
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

            if do_return {
                if let None = calls.pop() {
                    return Err(RuntimeError::CannotReturn);
                }
            } else if let Some(new_calls) = mabe_new_calls {
                calls.push(new_calls);
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
                self.stack.push(memory.insert(rst, false));
            }
            Command::Sub => {
                let right = self.pop_stack()?;
                let left = self.pop_stack()?;
                let rst = memory.get(&left) - memory.get(&right);
                memory.dec(&left);
                memory.dec(&right);
                self.stack.push(memory.insert(rst, false));
            }
            Command::Call => {
                let target = self.pop_stack()?;

                if !target.is_function() {
                    return Err(RuntimeError::InvalidFunction);
                }

                let function_address = target.get_address();
                let num_args = {
                    let function = memory.get_function(function_address);
                    function.get_args()
                };
                memory.dec(&target);

                current_calls.function_command_index += 1;

                if self.stack.len() < num_args {
                    return Err(RuntimeError::InvalidNumberOfArguments);
                }

                let new_calls = CallInfo {
                    function_memory_address: function_address,
                    function_command_index: 0,
                    num_arguments: num_args,
                    argument_stack_index: self.stack.len() - num_args,
                    locals: Vec::new(),
                };

                return Ok((Some(new_calls), false, None));
            }
            Command::LoadArgument(index) => {
                let stack_index = current_calls.argument_stack_index + index;
                let value = match self.stack.get(stack_index) {
                    Some(value) => value.clone(),
                    None => return Err(RuntimeError::CannotLoadStackArgument),
                };
                self.stack.push(value);
            }
            Command::Return => {
                let mut save = None;

                if self.stack.len() < current_calls.num_arguments {
                    return Err(RuntimeError::ArgumentsNotOnStack);
                }

                if self.stack.len() - current_calls.num_arguments
                    == current_calls.argument_stack_index + 1
                {
                    save = Some(self.pop_stack()?);
                } else if self.stack.len() - current_calls.num_arguments
                    != current_calls.argument_stack_index
                {
                    return Err(RuntimeError::ArgumentsNotOnStack);
                }

                for _ in 0..current_calls.num_arguments {
                    self.pop_stack()?;
                }

                if let Some(value) = save {
                    self.stack.push(value);
                }

                return Ok((None, true, None));
            }
            Command::PrintDebug => {
                let target = self.pop_stack()?;
                memory.dec(&target);
                println!("{:?}", memory.get(&target));
            }
            Command::Halt(exit_code) => return Ok((None, false, Some(exit_code))),
        };

        current_calls.function_command_index += 1;

        Ok((None, false, None))
    }
}
