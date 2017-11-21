
use super::super::controller::Controller;
use super::super::logger::Logger;
use super::command::Command;

pub struct CommandContainer<'a> {
    commands: &'a mut Vec<Command>,
}

impl<'a> CommandContainer<'a> {
    pub fn new(commands: &'a mut Vec<Command>) -> CommandContainer {
        CommandContainer { commands: commands }
    }

    pub fn add_command<T: Logger>(&mut self, controller: &mut Controller<T>, command: Command) {
        {
            controller.get_logger_mut().builder_add_command(&command);
        }
        self.commands.push(command);
    }

    pub fn get_commands(&self) -> &Vec<Command> {
        &self.commands
    }

    pub fn is_last_clear(&self) -> bool {
        if let Some(command) = self.commands.last() {
            return match *command {
                Command::ClearRegisters => true,
                _ => false,
            };
        };
        false
    }
}
