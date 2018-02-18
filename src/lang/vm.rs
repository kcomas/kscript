use super::command::Command;
use super::address::MemoryAddress;
use super::memory::Memory;
use super::data::Data;
use super::error::RuntimeError;

#[derive(Debug, Clone)]
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
            call.current_command_index += command_inc;
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
            Command::Equals => {
                let right = self.pop_stack()?;
                let left = self.pop_stack()?;

                let b = if left.is_int() && right.is_int() {
                    left.as_int() == right.as_int()
                } else {
                    return Err(RuntimeError::CannotCompareTypes);
                };

                memory.dec(&right)?;
                memory.dec(&left)?;

                self.stack.push(memory.insert_counted(Data::Bool(b)));
            }
            Command::JumpIfFalse(count) => {
                let target = self.pop_stack()?;

                if !target.get_bool()? {
                    self.update_current_call_index(count)?;
                    return Ok(None);
                }
            }
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
            Command::Call => {
                let target = self.pop_stack()?;

                {
                    let function = memory.get(&target)?;
                    let function = match function {
                        Some(data) => data.get_function()?,
                        None => return Err(RuntimeError::TargetIsNotAFunction),
                    };
                    self.update_current_call_index(1)?;

                    self.call_stack.push(Call {
                        current_command_index: function.entry_index,
                        entry_index: function.entry_index,
                        stack_index: self.stack.len(),
                        number_arguments: function.number_arguments,
                        number_locals: function.number_locals,
                    });
                }

                memory.dec(&target)?;
                return Ok(None);
            }
            Command::CallSelf => {
                let mut current_call_clone = self.get_current_call()?.clone();
                self.update_current_call_index(1)?;

                current_call_clone.current_command_index = current_call_clone.entry_index;
                current_call_clone.stack_index = self.stack.len();
                self.call_stack.push(current_call_clone);
                return Ok(None);
            }
            Command::LoadArg(index) => {
                let arg = {
                    let current_call = self.get_current_call()?;
                    let pos = current_call.stack_index - current_call.number_arguments + index;
                    match self.stack.get(pos) {
                        Some(arg) => arg.clone(),
                        None => return Err(RuntimeError::InvalidArgumentIndex),
                    }
                };
                memory.inc(&arg)?;
                self.stack.push(arg);
            }
            Command::Return => {
                let last_call = match self.call_stack.pop() {
                    Some(last_call) => last_call,
                    None => return Err(RuntimeError::CannotReturnFromFunction),
                };

                let mut save = None;

                if self.stack.len() == last_call.stack_index + last_call.number_locals + 1 {
                    save = Some(self.pop_stack()?);
                } else if self.stack.len() != last_call.stack_index + last_call.number_locals {
                    return Err(RuntimeError::InvalidStackReturnLength);
                }

                for _ in 0..last_call.number_arguments {
                    memory.dec(&self.pop_stack()?)?;
                }

                if let Some(item) = save {
                    self.stack.push(item);
                }

                return Ok(None);
            }
            Command::Print => {
                let target = self.pop_stack()?;
                match memory.get(&target)? {
                    Some(data) => println!("{:?}", data),
                    None => println!("{:?}", target),
                };
                memory.dec(&target)?;
            }
            Command::Halt(exit_code) => return Ok(Some(exit_code)),
        }

        self.update_current_call_index(1)?;
        Ok(None)
    }
}
