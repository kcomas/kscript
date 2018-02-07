use super::data_type::{DataType, FunctionPointer};
use super::command::Command;
use super::error::RuntimeError;

#[derive(Debug)]
pub struct Frame {
    return_index: usize,
    stack_index: usize,
    num_arguments: usize,
    num_locals: usize,
    length: usize,
}

#[derive(Debug)]
pub struct Vm {
    stack: Vec<DataType>,
    command_index: usize,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            stack: Vec::new(),
            command_index: 0,
        }
    }

    pub fn create_frames() -> Vec<Frame> {
        vec![
            Frame {
                return_index: 0,
                stack_index: 0,
                num_arguments: 0,
                num_locals: 0,
                length: 0,
            },
        ]
    }

    pub fn run(
        &mut self,
        entry: usize,
        commands: &Vec<Command>,
        frames: &mut Vec<Frame>,
    ) -> Result<i32, RuntimeError> {
        self.command_index = entry;
        while let Some(command) = commands.get(self.command_index) {
            let (mabe_exit_code, mabe_new_frame, mabe_return) = {
                let last_frame = match frames.last_mut() {
                    Some(last_frame) => last_frame,
                    None => return Err(RuntimeError::NoMoreFrames),
                };
                self.match_command(command, last_frame)?
            };
            if let Some(exit_code) = mabe_exit_code {
                return Ok(exit_code);
            }

            if let Some(new_frame) = mabe_new_frame {
                frames.push(new_frame);
            }

            if mabe_return {
                if let None = frames.pop() {
                    return Err(RuntimeError::CannotReturnFromFrame);
                }
            }
        }
        return Err(RuntimeError::NoMoreCommands);
    }

    pub fn match_command(
        &mut self,
        command: &Command,
        frame: &mut Frame,
    ) -> Result<(Option<i32>, Option<Frame>, bool), RuntimeError> {
        match *command {
            Command::PushStack(ref data_type) => self.stack.push(data_type.shallow_clone()),
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
            Command::Call => {
                let target = self.pop_stack()?;

                let function = target.get_function()?;
                let function = function.borrow();

                let return_index = self.command_index + 1;
                self.command_index = function.command_index;

                return Ok((
                    None,
                    Some(Vm::frame_from_function(
                        &*function,
                        return_index,
                        self.stack.len(),
                    )),
                    false,
                ));
            }
            Command::Return => {
                if frame.num_arguments > 0 {}
                if frame.num_locals > 0 {}

                self.command_index = frame.return_index;
                return Ok((None, None, true));
            }
            Command::Halt(exit_code) => return Ok((Some(exit_code), None, false)),
        };
        self.command_index += 1;
        Ok((None, None, false))
    }

    fn pop_stack(&mut self) -> Result<DataType, RuntimeError> {
        if let Some(data_type) = self.stack.pop() {
            return Ok(data_type);
        }
        Err(RuntimeError::StackEmpty)
    }

    fn frame_from_function(
        function: &FunctionPointer,
        return_index: usize,
        stack_index: usize,
    ) -> Frame {
        Frame {
            return_index: return_index,
            stack_index: stack_index,
            num_arguments: function.num_arguments,
            num_locals: function.num_locals,
            length: function.length,
        }
    }
}
