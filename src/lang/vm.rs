use super::memory::MemoryAddress;
use super::function::FunctionPointer;
use super::memory::Memory;
use super::data::DataHolder;
use super::command::Command;
use super::error::RuntimeError;

#[derive(Debug)]
pub struct Vm {
    stack: Vec<MemoryAddress>,
    stack_function_index: Vec<usize>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            stack: Vec::new(),
            stack_function_index: vec![0],
        }
    }

    pub fn init(&mut self, memory: &mut Memory, current_command_index: usize, num_locals: usize) {
        self.stack
            .push(memory.insert_counted(DataHolder::Function(FunctionPointer {
                current_command_index: current_command_index,
                num_arguments: 0,
                num_locals: num_locals,
            })));
    }

    pub fn run(
        &mut self,
        memory: &mut Memory,
        commands: &Vec<Command>,
    ) -> Result<i32, RuntimeError> {
        loop {
            let current_call = self.get_current_function_pointer()?;
            let command = match commands.get(current_call.current_command_index) {
                Some(command) => command,
                None => return Err(RuntimeError::InvalidCommandIndex),
            };

            let mabe_exit = self.match_command(memory, &current_call, command)?;

            if let Some(exit_code) = mabe_exit {
                return Ok(exit_code);
            }
        }
    }

    fn current_function_index(&self) -> Result<usize, RuntimeError> {
        if let Some(index) = self.stack_function_index.last() {
            return Ok(*index);
        }
        Err(RuntimeError::CannotGetLastFunctionIndex)
    }

    fn get_current_function_pointer(&self) -> Result<FunctionPointer, RuntimeError> {
        if let Some(item) = self.stack.get(self.current_function_index()?) {
            return Ok(item.get_function()?);
        }
        Err(RuntimeError::CannotLoadCurrentFunction)
    }

    fn uppdate_current_function_comamnd_index(
        &mut self,
        new_command_index: usize,
    ) -> Result<(), RuntimeError> {
        let last_index = self.current_function_index()?;
        if let Some(item) = self.stack.get_mut(last_index) {
            let function = item.get_function_mut()?;
            function.current_command_index = new_command_index;
            return Ok(());
        }
        Err(RuntimeError::CannotUpdateCurrentFunction)
    }

    fn pop_stack(&mut self) -> Result<MemoryAddress, RuntimeError> {
        if let Some(item) = self.stack.pop() {
            return Ok(item);
        }
        Err(RuntimeError::StackEmpty)
    }

    fn match_command(
        &mut self,
        memory: &mut Memory,
        current_call: &FunctionPointer,
        command: &Command,
    ) -> Result<Option<i32>, RuntimeError> {
        match *command {
            Command::Push(ref address) => self.stack.push(address.clone()),
            Command::Equals => {
                let right = self.pop_stack()?;
                let left = self.pop_stack()?;

                let b = {
                    let right = memory.get(&right)?;
                    let left = memory.get(&left)?;

                    if left.is_int() && right.is_int() {
                        left.as_int() == right.as_int()
                    } else {
                        return Err(RuntimeError::CannotCompareTypes);
                    }
                };

                memory.dec(&right)?;
                memory.dec(&left)?;
                self.stack.push(memory.insert_counted(DataHolder::Bool(b)));
            }
            Command::JumpIfFalse(index) => {
                let target = self.pop_stack()?;

                let b = target.get_bool()?;

                if !b {
                    self.uppdate_current_function_comamnd_index(
                        current_call.current_command_index + 1 + index,
                    )?;
                    return Ok(None);
                }
            }
            Command::Add => {
                let right = self.pop_stack()?;
                let left = self.pop_stack()?;

                let value = memory.get(&left)? + memory.get(&right)?;

                memory.dec(&right)?;
                memory.dec(&left)?;

                self.stack.push(memory.insert_counted(value));
            }
            Command::Sub => {
                let right = self.pop_stack()?;
                let left = self.pop_stack()?;

                let value = memory.get(&left)? - memory.get(&right)?;

                memory.dec(&right)?;
                memory.dec(&left)?;

                self.stack.push(memory.insert_counted(value));
            }
            Command::Call => {
                self.uppdate_current_function_comamnd_index(
                    current_call.current_command_index + 1,
                )?;
                self.stack_function_index.push(self.stack.len() - 1);
                return Ok(None);
            }
            Command::LoadArg(index) => {
                let stack_index = self.current_function_index()? - index;
                let arg = match self.stack.get(stack_index) {
                    Some(arg) => arg.clone(),
                    None => return Err(RuntimeError::CannotLoadArgument),
                };
                memory.inc(&arg)?;
                self.stack.push(arg);
            }
            Command::Return => {
                let mut save = None;

                let current_function_pointer = self.get_current_function_pointer()?;

                let last_index = self.current_function_index()?;

                if last_index + current_function_pointer.num_locals + 2 == self.stack.len() {
                    save = Some(self.pop_stack()?);
                } else if last_index + 1 + current_function_pointer.num_locals != self.stack.len() {
                    return Err(RuntimeError::InvalidRetrunLength);
                }

                for _ in 0..current_function_pointer.num_arguments {
                    memory.dec(&self.pop_stack()?)?;
                }

                memory.dec(&self.pop_stack()?)?;

                self.stack_function_index.pop();

                if let Some(value) = save {
                    self.stack.push(value);
                }

                return Ok(None);
            }
            Command::Print => {
                let target = self.pop_stack()?;
                println!("{:?}", memory.get(&target)?);
                memory.dec(&target)?;
            }
            Command::Halt(exit_code) => return Ok(Some(exit_code)),
        };
        self.uppdate_current_function_comamnd_index(current_call.current_command_index + 1)?;
        Ok(None)
    }
}
