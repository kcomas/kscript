
pub mod scope;

use super::controller::Controller;
use super::logger::Logger;
use super::error::Error;
use super::builder::BuilderRunner;
use super::builder::command::Command;
use self::scope::Scope;

pub struct Vm<'a, T: Logger + 'a> {
    controller: &'a mut Controller<T>,
}

impl<'a, T> Vm<'a, T>
where
    T: Logger,
{
    pub fn new(controller: &'a mut Controller<T>) -> Vm<'a, T> {
        Vm { controller: controller }
    }

    pub fn run(&mut self, commands: &Vec<Command>, scope: &mut Scope) -> Result<(), Error> {
        {
            self.controller.get_logger_mut().scope_enter();
        }
        for command in commands.iter() {
            self.match_command(command, scope)?;
        }
        {
            self.controller.get_logger_mut().scope_exit();
        }
        Ok(())
    }

    pub fn match_command(&mut self, command: &Command, scope: &mut Scope) -> Result<(), Error> {
        match *command {
            Command::SetRegister(ref reg, ref data_holder) => {
                self.controller.get_logger_mut().scope_set_register(
                    reg,
                    data_holder,
                );
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
