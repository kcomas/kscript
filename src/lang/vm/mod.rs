
pub mod scope;
pub mod vm_types;
mod util;
mod io;

use super::controller::Controller;
use super::logger::Logger;
use super::error::Error;
use super::builder::command::Command;
use self::scope::Scope;
use self::util::conditional_to_parts;
use self::io::{io_write, io_append};

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
            self.controller.get_logger_mut().scope_enter(scope.get_id());
        }
        for command in commands.iter() {
            let _ = self.match_command(command, scope)?;
        }
        {
            let logger = self.controller.get_logger_mut();
            logger.scope_dump(scope);
            logger.scope_exit(scope.get_id());
        }
        Ok(())
    }

    fn match_command(&mut self, command: &Command, scope: &mut Scope) -> Result<(), Error> {
        {
            self.controller.get_logger_mut().scope_run_command(command);
        }
        match *command {
            Command::SetRegister(reg, ref data_holder) => {
                scope.set_register(self.controller, reg, data_holder)?
            }
            Command::Assign(left, right) => scope.assign(left, right)?,
            Command::ClearRegisters => scope.clear_registers(),
            Command::IoWrite(left, right) => io_write(scope, left, right)?,
            Command::IoAppend(left, right) => io_append(scope, left, right)?,
            Command::Addition(sink, left, right) => scope.addition(sink, left, right)?,
            Command::Subtract(sink, left, right) => scope.subtract(sink, left, right)?,
            Command::Multiply(sink, left, right) => scope.multiply(sink, left, right)?,
            Command::Divide(sink, left, right) => scope.divide(sink, left, right)?,
            Command::Modulus(sink, left, right) => scope.modulus(sink, left, right)?,
            Command::Exponent(sink, left, right) => scope.exponent(sink, left, right)?,
            Command::If(ref conditional, ref true_commands, ref false_commands) => {
                let (left_data, cond, right_data) = conditional_to_parts(conditional)?;
                match scope.evaluate_conditional(
                    self.controller,
                    left_data,
                    cond,
                    right_data,
                )? {
                    true => self.run(true_commands, scope)?,
                    false => self.run(false_commands, scope)?,
                };
            }
            Command::Loop(ref conditional, ref commands) => {
                let (mut left_data, mut cond, mut right_data) = conditional_to_parts(conditional)?;
                while scope.evaluate_conditional(
                    self.controller,
                    left_data,
                    cond,
                    right_data,
                )?
                {
                    self.run(commands, scope)?;
                    let (l, c, r) = conditional_to_parts(conditional)?;
                    left_data = l;
                    cond = c;
                    right_data = r;
                }
            }
            _ => {}
        };
        Ok(())
    }
}
