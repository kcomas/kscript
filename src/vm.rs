use std::rc::Rc;
use super::command::{Command, SharedCommands};
use super::data_type::DataType;
use super::error::RuntimeError;

#[derive(Debug)]
struct CallInfo {
    pub commands: SharedCommands,
    pub num_args: usize,
    pub stack_index: usize,
    pub command_index: usize,
}

#[derive(Debug)]
pub struct Vm {
    locals: Vec<DataType>,
    stack: Vec<DataType>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            locals: Vec::new(),
            stack: Vec::new(),
        }
    }

    pub fn run(&mut self, commands: &SharedCommands) -> Result<i32, RuntimeError> {
        let mut calls: Vec<CallInfo> = vec![
            CallInfo {
                commands: Rc::clone(commands),
                num_args: 0,
                stack_index: 0,
                command_index: 0,
            },
        ];
        loop {
            let (mabe_new_calls, do_return, mabe_exit_code) = match calls.last_mut() {
                Some(ref mut current_calls) => self.match_command(current_calls)?,
                None => return Err(RuntimeError::CallsEmpty),
            };
            if let Some(code) = mabe_exit_code {
                return Ok(code);
            }
            if do_return {
                let rst = calls.pop();
                if let None = rst {
                    return Err(RuntimeError::CannotReturn);
                }
                continue;
            }
            if let Some(new_calls) = mabe_new_calls {
                calls.push(new_calls);
            }
        }
    }

    fn pop_stack(&mut self) -> Result<DataType, RuntimeError> {
        if let Some(data) = self.stack.pop() {
            return Ok(data);
        }
        Err(RuntimeError::StackEmpty)
    }

    fn match_command(
        &mut self,
        current_calls: &mut CallInfo,
    ) -> Result<(Option<CallInfo>, bool, Option<i32>), RuntimeError> {
        let command = match current_calls.commands.get(current_calls.command_index) {
            Some(ref_cmd) => ref_cmd.clone(),
            None => return Err(RuntimeError::NoMoreCommands),
        };
        match command {
            Command::PushStack(data) => self.stack.push(data),
            Command::SaveLocal(index) => {
                let value = self.pop_stack()?;
                if index < self.locals.len() {
                    self.locals[index] = value;
                } else if index == self.locals.len() {
                    self.locals.push(value);
                } else {
                    return Err(RuntimeError::InvalidLocalSaveIndex(index));
                }
            }
            Command::LoadLocal(index) => {
                let value = match self.locals.get(index) {
                    Some(value) => value.clone(),
                    None => return Err(RuntimeError::InvalidLocalGetIndex(index)),
                };
                self.stack.push(value);
            }
            Command::Call => {
                let function = self.pop_stack()?;
                let (body, num_args) = function.get_function()?;

                current_calls.command_index += 1;

                if self.stack.len() < num_args {
                    return Err(RuntimeError::InvalidNumberOfArguments);
                }

                let new_calls = CallInfo {
                    commands: body,
                    num_args: num_args,
                    stack_index: self.stack.len() - num_args,
                    command_index: 0,
                };

                return Ok((Some(new_calls), false, None));
            }
            Command::LoadStackArg(index) => {
                let stack_index = current_calls.stack_index + index;
                let value = match self.stack.get(stack_index) {
                    Some(val) => val.clone(),
                    None => return Err(RuntimeError::CannotLoadArgToStack(stack_index)),
                };
                self.stack.push(value);
            }
            Command::Return => {
                let mut save = None;
                if self.stack.len() < current_calls.num_args {
                    return Err(RuntimeError::ArgumentsNotOnStack(self.stack.len()));
                }

                if self.stack.len() - current_calls.num_args == current_calls.stack_index + 1 {
                    // save
                    save = Some(self.pop_stack()?);
                } else if self.stack.len() - current_calls.num_args != current_calls.stack_index {
                    return Err(RuntimeError::ArgumentsNotOnStack(self.stack.len()));
                }

                for _ in 0..current_calls.num_args {
                    self.pop_stack()?;
                }

                if let Some(value) = save {
                    self.stack.push(value);
                }

                return Ok((None, true, None));
            }
            Command::Halt(code) => return Ok((None, false, Some(code))),
        };
        current_calls.command_index += 1;
        Ok((None, false, None))
    }
}
