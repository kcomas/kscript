use std::rc::Rc;
use std::cell::RefCell;
use super::command::{Command, SharedCommands};
use super::data_type::DataType;
use super::error::RuntimeError;

#[derive(Debug)]
struct CallInfo {
    pub commands: SharedCommands,
    pub args: usize,
    pub stack_index: usize,
    pub command_index: usize,
}

#[derive(Debug)]
pub struct Vm {
    stack: Vec<DataType>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm { stack: Vec::new() }
    }

    pub fn run(&mut self, commands: &SharedCommands) -> Result<i32, RuntimeError> {
        let mut calls: Vec<CallInfo> = vec![
            CallInfo {
                commands: Rc::clone(commands),
                args: 0,
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
            Command::Halt(code) => return Ok((None, false, Some(code))),
            Command::Call => {
                let function = self.pop_stack()?;
                let (body, num_args) = function.get_function()?;

                current_calls.command_index += 1;

                let new_calls = CallInfo {
                    commands: body,
                    args: num_args,
                    stack_index: self.stack.len(),
                    command_index: 0,
                };

                return Ok((Some(new_calls), false, None));
            }
            Command::Return => return Ok((None, true, None)),
        };
        current_calls.command_index += 1;
        Ok((None, false, None))
    }
}
