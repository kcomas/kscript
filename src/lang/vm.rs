use std::rc::Rc;
use super::command::{Command, SharedCommands};
use super::data_type::{wrap_data, DataType, SharedData};
use super::error::RuntimeError;

#[derive(Debug)]
pub struct CallInfo {
    pub commands: SharedCommands,
    pub num_args: usize,
    pub stack_index: usize,
    pub command_index: usize,
    pub locals: Vec<SharedData>,
}

impl CallInfo {
    pub fn update_commands(&mut self, commands: &SharedCommands) {
        self.commands = Rc::clone(commands);
        self.command_index = 0;
    }
}

#[derive(Debug)]
pub struct Vm {
    stack: Vec<SharedData>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm { stack: Vec::new() }
    }

    pub fn create_calls(commands: &SharedCommands) -> Vec<CallInfo> {
        vec![
            CallInfo {
                commands: Rc::clone(commands),
                num_args: 0,
                stack_index: 0,
                command_index: 0,
                locals: Vec::new(),
            },
        ]
    }

    pub fn run(&mut self, calls: &mut Vec<CallInfo>) -> Result<i32, RuntimeError> {
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

    fn pop_stack(&mut self) -> Result<SharedData, RuntimeError> {
        if let Some(data) = self.stack.pop() {
            return Ok(data);
        }
        Err(RuntimeError::StackEmpty)
    }

    fn pop_borrow_clone(&mut self) -> Result<DataType, RuntimeError> {
        let data = self.pop_stack()?;
        let data = data.borrow();
        Ok(data.clone())
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
            Command::PushStack(data) => self.stack.push(wrap_data(data)),
            Command::SaveLocal(index) => {
                let value = self.pop_stack()?;
                if index < current_calls.locals.len() {
                    current_calls.locals[index] = wrap_data(value.borrow().clone());
                } else if index == current_calls.locals.len() {
                    current_calls.locals.push(wrap_data(value.borrow().clone()));
                } else {
                    return Err(RuntimeError::InvalidLocalSaveIndex(index));
                }
            }
            Command::LoadLocal(index) => {
                let value = match current_calls.locals.get(index) {
                    Some(value) => value.clone(),
                    None => return Err(RuntimeError::InvalidLocalGetIndex(index)),
                };
                self.stack.push(value);
            }
            Command::Equals => {
                let right = self.pop_stack()?;
                let right = right.borrow();
                let left = self.pop_stack()?;
                let left = left.borrow();

                let b = if left.is_bool() && right.is_bool() {
                    left.as_bool() == right.as_bool()
                } else if left.is_int() && right.is_int() {
                    left.as_int() == right.as_int()
                } else if left.is_float() && right.is_float() {
                    left.as_float() == right.as_float()
                } else {
                    return Err(RuntimeError::CannotCompareTypes(
                        left.clone(),
                        right.clone(),
                    ));
                };

                self.stack.push(wrap_data(DataType::Bool(b)));
            }
            Command::Add => {
                let right = self.pop_borrow_clone()?;
                let left = self.pop_borrow_clone()?;
                self.stack.push(wrap_data(left + right));
            }
            Command::Sub => {
                let right = self.pop_borrow_clone()?;
                let left = self.pop_borrow_clone()?;
                self.stack.push(wrap_data(left - right));
            }
            Command::Mul => {
                let right = self.pop_borrow_clone()?;
                let left = self.pop_borrow_clone()?;
                self.stack.push(wrap_data(left * right));
            }
            Command::Div => {
                let right = self.pop_borrow_clone()?;
                let left = self.pop_borrow_clone()?;
                self.stack.push(wrap_data(left / right));
            }
            Command::Rem => {
                let right = self.pop_borrow_clone()?;
                let left = self.pop_borrow_clone()?;
                self.stack.push(wrap_data(left % right));
            }
            Command::Exp => {
                let right = self.pop_stack()?;
                let right = right.borrow();
                let left = self.pop_stack()?;
                let left = left.borrow();

                let value = if left.is_int() && right.is_int() {
                    DataType::Integer(left.as_int().pow(right.as_int() as u32))
                } else if left.is_float() && right.is_int() {
                    DataType::Float(left.as_float().powi(right.as_int() as i32))
                } else {
                    DataType::Float(left.as_float().powf(right.as_float()))
                };
                self.stack.push(wrap_data(value));
            }
            Command::Concat => {
                let right = self.pop_stack()?;
                let right = right.borrow();
                let left = self.pop_stack()?;
                let mut left = left.borrow_mut();
                if left.is_string() && right.is_string() {
                    let right = right.as_str();
                    let mut left = left.as_string_mut()?;
                    left.push_str(right);
                } else {
                    return Err(RuntimeError::CannotConcat(left.clone(), right.clone()));
                }
            }
            Command::JumpIfFalse(to) => {
                let cmp = self.pop_stack()?;
                let cmp = cmp.borrow();

                if !cmp.get_bool()? {
                    current_calls.command_index += to;
                    return Ok((None, false, None));
                }
            }
            Command::Call => {
                let function = self.pop_stack()?;
                let function = function.borrow();
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
                    locals: Vec::new(),
                };

                return Ok((Some(new_calls), false, None));
            }
            Command::CallSelf => {
                let num_args = current_calls.num_args;

                current_calls.command_index += 1;

                if self.stack.len() < num_args {
                    return Err(RuntimeError::InvalidNumberOfArguments);
                }

                let new_calls = CallInfo {
                    commands: Rc::clone(&current_calls.commands),
                    num_args: num_args,
                    stack_index: self.stack.len() - num_args,
                    command_index: 0,
                    locals: Vec::new(),
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
            Command::SaveStackArg(index) => {
                let value = self.pop_stack()?;
                let stack_index = current_calls.stack_index + index;
                let stack_item = match self.stack.get(stack_index) {
                    Some(stack_item) => stack_item,
                    None => return Err(RuntimeError::CannotSaveToStackIndex(stack_index)),
                };
                *stack_item.borrow_mut() = value.borrow().clone();
            }
            Command::Return => {
                let mut save = None;
                if self.stack.len() < current_calls.num_args {
                    return Err(RuntimeError::ArgumentsNotOnStack(
                        self.stack.len(),
                        current_calls.num_args,
                    ));
                }

                if self.stack.len() - current_calls.num_args == current_calls.stack_index + 1 {
                    // save
                    save = Some(self.pop_stack()?);
                } else if self.stack.len() - current_calls.num_args != current_calls.stack_index {
                    return Err(RuntimeError::ArgumentsNotOnStack(
                        self.stack.len(),
                        self.stack.len() - current_calls.num_args,
                    ));
                }

                for _ in 0..current_calls.num_args {
                    self.pop_stack()?;
                }

                if let Some(value) = save {
                    self.stack.push(value);
                }

                return Ok((None, true, None));
            }
            Command::IoWrite => {
                let target = self.pop_stack()?;
                let target = target.borrow();
                let value = self.pop_stack()?;
                let value = value.borrow();
                if target.is_int() {
                    match target.as_int() {
                        1 => print!("{}", value),
                        2 => eprint!("{}", value),
                        _ => return Err(RuntimeError::InvalidIoAppendTarget(target.clone())),
                    }
                }
            }
            Command::IoAppend => {
                let target = self.pop_stack()?;
                let target = target.borrow();
                let value = self.pop_stack()?;
                let value = value.borrow();
                if target.is_int() {
                    match target.as_int() {
                        1 => println!("{}", value),
                        2 => eprintln!("{}", value),
                        _ => return Err(RuntimeError::InvalidIoAppendTarget(target.clone())),
                    }
                }
            }
            Command::Halt(code) => return Ok((None, false, Some(code))),
        };
        current_calls.command_index += 1;
        Ok((None, false, None))
    }
}
