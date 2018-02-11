use super::memory::{Memory, MemoryAddress};
use super::command::Command;
use super::function::FunctionPointer;
use super::data::DataHolder;
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
                num_args: 0,
                num_locals: 0,
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

            if let Some(call) = mabe_call {
                call_stack.push(call);
            }

            if mabe_return {
                if let None = call_stack.pop() {
                    return Err(RuntimeError::InvalidReturn);
                }
            }
        }
    }

    fn match_command(
        &mut self,
        command: &Command,
        memory: &mut Memory,
        current_call: &mut FunctionPointer,
    ) -> Result<(Option<i32>, Option<FunctionPointer>, bool), RuntimeError> {
        match *command {
            Command::PushStack(ref address) => self.stack.push(address.clone()),
            Command::Equals => {
                let right = self.pop_stack()?;
                let left = self.pop_stack()?;
                let right = memory.pop(&right)?;
                let left = memory.pop(&left)?;

                let b = if left.is_int() && right.is_int() {
                    left.as_int() == right.as_int()
                } else {
                    return Err(RuntimeError::CannotCompareTypes);
                };

                self.stack.push(memory.insert_stack(DataHolder::Bool(b)));
            }
            Command::JumpIfFalse(skip) => {
                let target = self.pop_stack()?;
                let target = memory.pop(&target)?;
                if !target.get_bool()? {
                    current_call.current_command_index += skip;
                }
            }
            Command::Add => {
                let right = self.pop_stack()?;
                let left = self.pop_stack()?;
                let right = memory.pop(&right)?;
                let left = memory.pop(&left)?;
                self.stack.push(memory.insert_stack(left + right));
            }
            Command::Sub => {
                let right = self.pop_stack()?;
                let left = self.pop_stack()?;
                let right = memory.pop(&right)?;
                let left = memory.pop(&left)?;
                self.stack.push(memory.insert_stack(left - right));
            }
            Command::Call => {
                let target = self.pop_stack()?;
                let function = memory.pop(&target)?;
                let mut function = function.get_function()?.clone();
                function.entry_stack_len = self.stack.len();
                current_call.current_command_index += 1;
                return Ok((None, Some(function), false));
            }
            Command::CallSelf => {
                let mut function = current_call.clone();
                function.entry_stack_len = self.stack.len();
                function.current_command_index = current_call.entry_command_index;
                current_call.current_command_index += 1;
                return Ok((None, Some(function), false));
            }
            Command::LoadArg(index) => {
                let stack_index = current_call.entry_stack_len - current_call.num_args + index;
                let value = match self.stack.get(stack_index) {
                    Some(value) => {
                        let arg = memory.get(value)?;
                        memory.insert_stack(arg)
                    }
                    None => return Err(RuntimeError::CannotLoadArgument),
                };
                self.stack.push(value)
            }
            Command::Return => {
                let mut save = None;

                if self.stack.len() == current_call.entry_stack_len + 1 {
                    save = Some(memory.pop(&self.pop_stack()?)?);
                } else if self.stack.len() != current_call.entry_stack_len {
                    return Err(RuntimeError::InvalidReturnStackLen);
                }

                for _ in 0..current_call.num_locals {
                    memory.pop(&self.pop_stack()?)?;
                }

                for _ in 0..current_call.num_args {
                    memory.pop(&self.pop_stack()?)?;
                }

                if let Some(value) = save {
                    self.stack.push(memory.insert_stack(value));
                }

                return Ok((None, None, true));
            }
            Command::Print => {
                let target = self.pop_stack()?;
                let target = memory.pop(&target)?;
                println!("{:?}", target);
            }
            Command::Halt(exit_code) => return Ok((Some(exit_code), None, false)),
        };
        current_call.current_command_index += 1;
        Ok((None, None, false))
    }

    fn pop_stack(&mut self) -> Result<MemoryAddress, RuntimeError> {
        if let Some(item) = self.stack.pop() {
            return Ok(item);
        }
        Err(RuntimeError::VmStackEmpty)
    }
}
