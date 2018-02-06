use super::data_type::DataType;
use super::command::Command;
use super::error::RuntimeError;

#[derive(Debug)]
pub struct Frame {
    return_index: usize,
    num_arguments: usize,
    locals: Vec<DataType>,
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
                num_arguments: 0,
                locals: Vec::new(),
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
            let last_frame = match frames.last_mut() {
                Some(last_frame) => last_frame,
                None => return Err(RuntimeError::NoMoreFrames),
            };
            let (mabe_exit_code, mabe_new_frame) = self.match_command(command, last_frame)?;

            if let Some(exit_code) = mabe_exit_code {
                return Ok(exit_code);
            }
        }
        return Err(RuntimeError::NoMoreCommands);
    }

    pub fn match_command(
        &mut self,
        command: &Command,
        frame: &mut Frame,
    ) -> Result<(Option<i32>, Option<Frame>), RuntimeError> {
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
            Command::Halt(exit_code) => return Ok((Some(exit_code), None)),
        };
        self.command_index += 1;
        Ok((None, None))
    }

    fn pop_stack(&mut self) -> Result<DataType, RuntimeError> {
        if let Some(data_type) = self.stack.pop() {
            return Ok(data_type);
        }
        Err(RuntimeError::StackEmpty)
    }
}
