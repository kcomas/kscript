use std::rc::Rc;
use super::data_type::{wrap_type, DataType, SharedDataType};
use super::command::Command;
use super::error::Error;

#[derive(Debug)]
pub struct FunctionInfo {
    pub return_index: usize,
    pub stack_position: usize,
    pub num_args: usize,
}

#[derive(Debug)]
pub struct Vm {
    stack: Vec<SharedDataType>,
    // vars
    locals: Vec<Vec<SharedDataType>>,
    // the position to go to after a return
    function_return: Vec<FunctionInfo>,
}

impl<'a> Vm {
    pub fn new() -> Vm {
        Vm {
            stack: Vec::new(),
            locals: Vec::new(),
            function_return: Vec::new(),
        }
    }

    pub fn run(&mut self, commands: &Vec<Command>, entry: usize) -> Result<i32, Error<'a>> {
        let mut current_command_index = entry;
        loop {
            let mabe_command = commands.get(current_command_index);
            if let Some(command) = mabe_command {
                let (new_command_index, mabe_exit) =
                    self.match_command(command, current_command_index)?;
                if let Some(exit_code) = mabe_exit {
                    return Ok(exit_code);
                }
                current_command_index = new_command_index;
            } else {
                return Err(Error::InvalidCommandIndex(
                    current_command_index,
                    "No command at this index",
                ));
            }
        }
    }

    fn pop_stack(&mut self) -> Result<SharedDataType, Error<'a>> {
        if let Some(ref data_type) = self.stack.pop() {
            return Ok(Rc::clone(data_type));
        }
        Err(Error::StackEmpty("No more values on stack"))
    }

    fn pop_function_return(&mut self) -> Result<FunctionInfo, Error<'a>> {
        if let Some(function_data) = self.function_return.pop() {
            return Ok(function_data);
        }
        Err(Error::CannotReturn("No return position specified"))
    }

    fn last_local(&self) -> Result<&Vec<SharedDataType>, Error<'a>> {
        if let Some(current_locals) = self.locals.last() {
            return Ok(current_locals);
        }
        Err(Error::CannotGetLastLocals("No locals on the local stack"))
    }

    fn last_local_mut(&mut self) -> Result<&mut Vec<SharedDataType>, Error<'a>> {
        if let Some(current_locals) = self.locals.last_mut() {
            return Ok(current_locals);
        }
        Err(Error::CannotGetLastLocals("No locals on the local stack"))
    }

    fn match_command(
        &mut self,
        command: &Command,
        current_command_index: usize,
    ) -> Result<(usize, Option<i32>), Error<'a>> {
        match *command {
            Command::AddLocals => {
                self.locals.push(Vec::new());
            }
            Command::RemoveLocals => {
                self.locals.pop();
            }
            Command::Push(ref data_type) => self.stack.push(data_type.clone()),
            Command::Load(index) => {
                let mut new_data = None;
                if let Some(function_data) = self.function_return.last() {
                    if index < function_data.num_args {
                        if function_data.num_args > function_data.stack_position {
                            // no more on stack
                            return Err(Error::StackEmpty(
                                "Cannot load non existant data from stack",
                            ));
                        }
                        if let Some(ref data_type) = self.stack
                            .get(function_data.stack_position - function_data.num_args + index)
                        {
                            new_data = Some(Rc::clone(data_type));
                        } else {
                            return Err(Error::InvalidFunctionArgument(
                                index,
                                "Invalid function argument",
                            ));
                        }
                    } else {
                        // load from locals
                        // index - num_args is local position
                        if let Some(data) = self.last_local()?.get(index - function_data.num_args) {
                            new_data = Some(Rc::clone(data));
                        }
                    }
                } else {
                    // load from the locals
                    if let Some(data) = self.last_local()?.get(index) {
                        new_data = Some(Rc::clone(data));
                    }
                }
                if let Some(data) = new_data {
                    self.stack.push(data);
                }
            }
            Command::Save(index) => {
                let to_save = self.pop_stack()?;
                let to_save = to_save.borrow().clone();
                let mut local_index = index;
                if let Some(function_data) = self.function_return.last() {
                    if index < function_data.num_args {
                        // update the current stack
                        if let Some(data_type) = self.stack
                            .get_mut(function_data.stack_position - function_data.num_args + index)
                        {
                            *data_type.borrow_mut() = to_save;
                            return Ok((current_command_index + 1, None));
                        } else {
                            return Err(Error::CannotSave(
                                function_data.stack_position - function_data.num_args + index,
                                "No data in stack",
                            ));
                        }
                    } else {
                        local_index = index - function_data.num_args;
                    }
                }
                let current_local = self.last_local_mut()?;
                if local_index < current_local.len() {
                    *current_local[local_index].borrow_mut() = to_save;
                } else if local_index == current_local.len() {
                    current_local.push(wrap_type(to_save));
                } else {
                    return Err(Error::CannotSave(
                        local_index,
                        "Local index is greater then the local size",
                    ));
                }
            }
            Command::Equals => {
                let right = self.pop_stack()?;
                let right = right.borrow();
                let left = self.pop_stack()?;
                let left = left.borrow();

                let b;

                if left.is_int() && right.is_int() {
                    b = left.get_int() == right.get_int();
                } else if left.is_float() && right.is_float() {
                    b = left.get_float() == right.get_float();
                } else if left.is_bool() && right.is_bool() {
                    b = left.get_bool() == right.get_bool()
                } else {
                    return Err(Error::CannotCompare(
                        left.clone(),
                        right.clone(),
                        "Cannot compare types",
                    ));
                }
                self.stack.push(wrap_type(DataType::Bool(b)));
            }
            Command::Add => {
                let right = self.pop_stack()?;
                let right = right.borrow();
                let left = self.pop_stack()?;
                let left = left.borrow();
                self.stack.push(wrap_type(left.clone() + right.clone()));
            }
            Command::Sub => {
                let right = self.pop_stack()?;
                let right = right.borrow();
                let left = self.pop_stack()?;
                let left = left.borrow();
                self.stack.push(wrap_type(left.clone() - right.clone()));
            }
            Command::Mul => {
                let right = self.pop_stack()?;
                let right = right.borrow();
                let left = self.pop_stack()?;
                let left = left.borrow();
                self.stack.push(wrap_type(left.clone() * right.clone()));
            }
            Command::Exp => {
                let right = self.pop_stack()?;
                let right = right.borrow();
                let left = self.pop_stack()?;
                let left = left.borrow();
                if !left.is_number() || !right.is_number() {
                    return Err(Error::CannotExp(
                        left.clone(),
                        right.clone(),
                        "Cannot exp non number",
                    ));
                }
                if left.is_int() && right.is_int() {
                    self.stack.push(wrap_type(DataType::Integer(
                        left.get_int().pow(right.get_int() as u32),
                    )));
                } else if left.is_float() && right.is_int() {
                    self.stack.push(wrap_type(DataType::Float(
                        left.get_float().powi(right.get_int() as i32),
                    )));
                } else {
                    self.stack.push(wrap_type(DataType::Float(
                        left.get_float().powf(right.get_float()),
                    )));
                }
            }
            Command::Div => {
                let right = self.pop_stack()?;
                let right = right.borrow();
                let left = self.pop_stack()?;
                let left = left.borrow();
                self.stack.push(wrap_type(left.clone() / right.clone()));
            }
            Command::Rem => {
                let right = self.pop_stack()?;
                let right = right.borrow();
                let left = self.pop_stack()?;
                let left = left.borrow();
                self.stack.push(wrap_type(left.clone() % right.clone()));
            }
            Command::IoWrite => {
                let target = self.pop_stack()?;
                let target = target.borrow();
                let source = self.pop_stack()?;
                let source = source.borrow();
                if target.is_int() {
                    match target.get_int() {
                        1 => print!("{}", source),
                        2 => eprint!("{}", source),
                        _ => {
                            return Err(Error::InvalidWriteTarget(
                                target.clone(),
                                "Cannot write to fd",
                            ))
                        }
                    }
                }
            }
            Command::IoAppend => {
                let target = self.pop_stack()?;
                let target = target.borrow();
                let source = self.pop_stack()?;
                let source = source.borrow();
                if target.is_int() {
                    match target.get_int() {
                        1 => println!("{}", source),
                        2 => eprintln!("{}", source),
                        _ => {
                            return Err(Error::InvalidWriteTarget(
                                target.clone(),
                                "Cannot append to fd",
                            ))
                        }
                    }
                }
            }
            Command::Jmpf(index) => {
                let cmp = self.pop_stack()?;
                let cmp = cmp.borrow();
                if !cmp.get_bool() {
                    return Ok((index, None));
                }
            }
            Command::Call(args, index) => {
                let function_data = FunctionInfo {
                    return_index: current_command_index + 1,
                    stack_position: self.stack.len(),
                    num_args: args,
                };
                self.function_return.push(function_data);
                return Ok((index, None));
            }
            Command::Return => {
                let function_data = self.pop_function_return()?;
                let mut restore = None;
                if self.stack.len() > function_data.stack_position {
                    restore = self.stack.pop();
                }
                for _ in 0..function_data.num_args {
                    self.pop_stack()?;
                }
                if let Some(res) = restore {
                    self.stack.push(res);
                }
                return Ok((function_data.return_index, None));
            }
            Command::Halt(code) => return Ok((0, Some(code))),
        };
        Ok((current_command_index + 1, None))
    }
}
