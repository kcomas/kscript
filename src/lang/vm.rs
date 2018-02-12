use super::memory::MemoryAddress;
use super::function::FunctionPointer;
use super::command::Command;
use super::memory::Memory;
use super::error::RuntimeError;
use super::data::DataHolder;

#[derive(Debug)]
pub struct Vm {
    stack: Vec<MemoryAddress>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm { stack: Vec::new() }
    }

    pub fn create_call_stack(num_locals: usize) -> Vec<FunctionPointer> {
        vec![
            FunctionPointer {
                entry_command_index: 0,
                current_command_index: 0,
                num_arguments: 0,
                num_locals: num_locals,
                entry_stack_len: 0,
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
            let (mabe_exit, mabe_call, mabe_return) = {
                let current_call = match call_stack.last_mut() {
                    Some(current_call) => current_call,
                    None => return Err(RuntimeError::CallStackEmpty),
                };

                let command = match commands.get(current_call.current_command_index) {
                    Some(command) => command,
                    None => return Err(RuntimeError::InvalidCommandIndex),
                };

                self.match_command(command, memory, current_call)?
            };

            if let Some(exit_code) = mabe_exit {
                return Ok(exit_code);
            }

            if let Some(new_call) = mabe_call {
                call_stack.push(new_call);
            }

            if mabe_return {
                if let None = call_stack.pop() {
                    return Err(RuntimeError::CallStackEmpty);
                }
            }
        }
    }

    pub fn clean_locals(
        &mut self,
        memory: &mut Memory,
        current_call: &FunctionPointer,
    ) -> Result<(), RuntimeError> {
        for _ in 0..current_call.num_locals {
            memory.dec(&self.pop_stack()?)?;
        }
        Ok(())
    }

    fn pop_stack(&mut self) -> Result<MemoryAddress, RuntimeError> {
        if let Some(address) = self.stack.pop() {
            return Ok(address);
        }
        Err(RuntimeError::StackEmpty)
    }

    fn match_command(
        &mut self,
        command: &Command,
        memory: &mut Memory,
        current_call: &mut FunctionPointer,
    ) -> Result<(Option<i32>, Option<FunctionPointer>, bool), RuntimeError> {
        match *command {
            Command::PushStack(ref address) => self.stack.push(address.clone()),
            Command::Add => {
                let right = self.pop_stack()?;
                let left = self.pop_stack()?;
                let value = {
                    let right = memory.get(&right)?;
                    let left = memory.get(&left)?;
                    left + right
                };
                memory.dec(&right)?;
                memory.dec(&left)?;
                self.stack.push(memory.insert_dynamic(value));
            }
            Command::Equals => {
                let right = self.pop_stack()?;
                let left = self.pop_stack()?;

                let b = {
                    let right = memory.get(&right)?;
                    let left = memory.get(&left)?;

                    if left.is_int() && right.is_int() {
                        left.as_int() == right.as_int()
                    } else {
                        return Err(RuntimeError::CannotCompareDifferentTypes);
                    }
                };
                memory.dec(&right)?;
                memory.dec(&left)?;
                self.stack.push(memory.insert_dynamic(DataHolder::Bool(b)));
            }
            Command::JumpIfFalse(skip) => {
                let target = self.pop_stack()?;

                {
                    if !memory.get(&target)?.get_bool()? {
                        current_call.current_command_index += skip;
                    }
                }
                memory.dec(&target)?;
            }
            Command::Sub => {
                let right = self.pop_stack()?;
                let left = self.pop_stack()?;
                let value = {
                    let right = memory.get(&right)?;
                    let left = memory.get(&left)?;
                    left - right
                };
                memory.dec(&right)?;
                memory.dec(&left)?;
                self.stack.push(memory.insert_dynamic(value));
            }
            Command::Call => {
                let target = self.pop_stack()?;

                let fn_call = {
                    let mut function = {
                        let data = memory.get(&target)?;
                        data.get_function()?.clone()
                    };
                    function.entry_stack_len = self.stack.len();
                    function
                };

                memory.dec(&target)?;

                current_call.current_command_index += 1;
                return Ok((None, Some(fn_call), false));
            }
            Command::CallSelf => {
                let mut fn_call = current_call.clone();
                fn_call.current_command_index = current_call.entry_command_index;
                fn_call.entry_stack_len = self.stack.len();
                current_call.current_command_index += 1;
                return Ok((None, Some(fn_call), false));
            }
            Command::LoadArgument(index) => {
                let stack_index = current_call.entry_stack_len - current_call.num_arguments + index;
                let value = match self.stack.get(stack_index) {
                    Some(value) => value.clone(),
                    None => return Err(RuntimeError::CannotLoadArgument),
                };
                memory.inc(&value)?;
                self.stack.push(value);
            }
            Command::Return => {
                let mut save = None;

                if self.stack.len() == current_call.entry_stack_len + 1 {
                    save = Some(self.pop_stack()?);
                } else if self.stack.len() != current_call.entry_stack_len {
                    return Err(RuntimeError::InvalidReturnStack);
                }

                self.clean_locals(memory, current_call)?;

                for _ in 0..current_call.num_arguments {
                    memory.dec(&self.pop_stack()?)?;
                }

                if let Some(value) = save {
                    self.stack.push(value);
                }

                return Ok((None, None, true));
            }
            Command::Print => {
                let target = self.pop_stack()?;
                {
                    println!("{:?}", memory.get(&target)?);
                }
                memory.dec(&target)?;
            }
            Command::Halt(exit_code) => return Ok((Some(exit_code), None, false)),
        }
        current_call.current_command_index += 1;
        Ok((None, None, false))
    }
}
