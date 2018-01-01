use super::data_type::DataType;
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
    stack: Vec<DataType>,
    // vars
    locals: Vec<Vec<DataType>>,
    // the position to go to after a return
    function_return: Vec<FunctionInfo>,
}

impl<'a> Vm {
    pub fn new() -> Vm {
        Vm {
            stack: Vec::new(),
            locals: vec![Vec::new()],
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
            // println!("{:?} {:?}", command, self);
            } else {
                return Err(Error::InvalidCommandIndex(
                    current_command_index,
                    "No command at this index",
                ));
            }
        }
    }

    fn pop_stack(&mut self) -> Result<DataType, Error<'a>> {
        if let Some(data_type) = self.stack.pop() {
            return Ok(data_type);
        }
        Err(Error::StackEmpty("No more values on stack"))
    }

    fn pop_function_return(&mut self) -> Result<FunctionInfo, Error<'a>> {
        if let Some(function_data) = self.function_return.pop() {
            return Ok(function_data);
        }
        Err(Error::CannotReturn("No return position specified"))
    }

    fn last_local(&self) -> Result<&Vec<DataType>, Error<'a>> {
        if let Some(current_locals) = self.locals.last() {
            return Ok(current_locals);
        }
        Err(Error::CannotGetLastLocals("No locals on the local stack"))
    }

    fn last_local_mut(&mut self) -> Result<&mut Vec<DataType>, Error<'a>> {
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
                        if let Some(data_type) = self.stack
                            .get(function_data.stack_position - function_data.num_args + index)
                        {
                            new_data = Some(data_type.clone());
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
                            new_data = Some(data.clone());
                        }
                    }
                } else {
                    // load from the locals
                    if let Some(data) = self.last_local()?.get(index) {
                        new_data = Some(data.clone());
                    }
                }
                if let Some(data) = new_data {
                    self.stack.push(data);
                }
            }
            Command::Save(index) => {
                let to_save = self.pop_stack()?;
                let mut local_index = index;
                if let Some(function_data) = self.function_return.last() {
                    if index < function_data.num_args {
                        // update the current stack
                        if let Some(data_type) = self.stack
                            .get_mut(function_data.stack_position - function_data.num_args + index)
                        {
                            *data_type = to_save;
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
                    current_local[local_index] = to_save;
                } else if local_index == current_local.len() {
                    current_local.push(to_save);
                } else {
                    return Err(Error::CannotSave(
                        local_index,
                        "Local index is greater then the local size",
                    ));
                }
            }
            Command::Equals => {
                let right = self.pop_stack()?;
                let left = self.pop_stack()?;

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
                self.stack.push(DataType::Bool(b));
            }
            Command::Add => {
                let right = self.pop_stack()?;
                let left = self.pop_stack()?;
                self.stack.push(left + right);
            }
            Command::Sub => {
                let right = self.pop_stack()?;
                let left = self.pop_stack()?;
                self.stack.push(left - right);
            }
            Command::Mul => {
                let right = self.pop_stack()?;
                let left = self.pop_stack()?;
                self.stack.push(left * right);
            }
            Command::Exp => {
                let right = self.pop_stack()?;
                let left = self.pop_stack()?;
                if !left.is_number() || !right.is_number() {
                    return Err(Error::CannotExp(
                        left.clone(),
                        right.clone(),
                        "Cannot exp non number",
                    ));
                }
                if left.is_int() && right.is_int() {
                    self.stack.push(DataType::Integer(
                        left.get_int().pow(right.get_int() as u32),
                    ));
                } else if left.is_float() && right.is_int() {
                    self.stack.push(DataType::Float(
                        left.get_float().powi(right.get_int() as i32),
                    ));
                } else {
                    self.stack
                        .push(DataType::Float(left.get_float().powf(right.get_float())));
                }
            }
            Command::Div => {
                let right = self.pop_stack()?;
                let left = self.pop_stack()?;
                self.stack.push(left / right);
            }
            Command::Rem => {
                let right = self.pop_stack()?;
                let left = self.pop_stack()?;
                self.stack.push(left % right);
            }
            Command::IoWrite => {
                let target = self.pop_stack()?;
                let source = self.pop_stack()?;
                if target.is_int() {
                    match target.get_int() {
                        1 => print!("{}", source),
                        2 => eprint!("{}", source),
                        _ => return Err(Error::InvalidWriteTarget(target, "Cannot write to fd")),
                    }
                }
            }
            Command::IoAppend => {
                let target = self.pop_stack()?;
                let source = self.pop_stack()?;
                if target.is_int() {
                    match target.get_int() {
                        1 => println!("{}", source),
                        2 => eprintln!("{}", source),
                        _ => return Err(Error::InvalidWriteTarget(target, "Cannot append to fd")),
                    }
                }
            }
            Command::Jmpf(index) => {
                let cmp = self.pop_stack()?;
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
                self.locals.push(Vec::new());
                self.function_return.push(function_data);
                return Ok((index, None));
            }
            Command::Return => {
                self.locals.pop();
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
