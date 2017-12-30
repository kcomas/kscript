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
    locals: Vec<DataType>,
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

    pub fn run(&mut self, commands: &Vec<Command>, entry: usize) -> Result<usize, Error<'a>> {
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

    fn match_command(
        &mut self,
        command: &Command,
        current_command_index: usize,
    ) -> Result<(usize, Option<usize>), Error<'a>> {
        match *command {
            Command::Push(ref data_type) => self.stack.push(data_type.clone()),
            Command::Load(index) => {
                let mut new_data = None;
                if let Some(function_data) = self.function_return.last() {
                    if index < function_data.num_args {
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
                        if let Some(data) = self.locals.get(index - function_data.num_args) {
                            new_data = Some(data.clone());
                        }
                    }
                } else {
                    // load from the locals
                    if let Some(data) = self.locals.get(index) {
                        new_data = Some(data.clone());
                    }
                }
                if let Some(data) = new_data {
                    self.stack.push(data);
                }
            }
            Command::Equals => {
                let right = self.pop_stack()?;
                let left = self.pop_stack()?;

                let b;

                if left.is_int() && right.is_int() {
                    b = left.get_int() == right.get_int();
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
                self.function_return.push(function_data);
                return Ok((index, None));
            }
            Command::Return => {
                self.locals.clear();
                let function_data = self.pop_function_return()?;
                let total_args = self.stack.len() - function_data.stack_position;
                let mut restore = None;
                if total_args != function_data.num_args {
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