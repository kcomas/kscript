
pub mod command;
pub mod command_container;
pub mod sub_builder;
pub mod run_builder;
pub mod assign_builder;
mod util;

use super::controller::Controller;
use super::logger::Logger;
use super::error::Error;
use super::parser::token_container::TokenContainer;
use self::command_container::CommandContainer;
use self::util::{set_type_registers, set_operator_registers, top_level_builders};
use self::command::Command;

pub struct BuilderRunner<'a, T: Logger + 'a> {
    controller: &'a mut Controller<T>,
}

impl<'a, T> BuilderRunner<'a, T>
where
    T: Logger,
{
    pub fn new(controller: &'a mut Controller<T>) -> BuilderRunner<'a, T> {
        BuilderRunner { controller: controller }
    }

    pub fn run(&mut self, token_container: &mut TokenContainer) -> Result<(), Error> {
        {
            self.controller.get_logger_mut().builder_start();
        }

        let mut command_container = CommandContainer::new();

        let (mut builders, num_builders) = top_level_builders();

        while !token_container.is_done() {
            // check if the token is an operator
            if token_container.is_current_token_end() {
                token_container.update_slice_end();
                let mut current_register: usize = 0;
                set_type_registers(
                    self.controller,
                    token_container,
                    &mut command_container,
                    &mut current_register,
                )?;
                token_container.reset_slice_position();
                // set operators
                set_operator_registers(
                    self.controller,
                    token_container,
                    &mut command_container,
                    &mut current_register,
                    &mut builders,
                    num_builders,
                )?;
                {
                    command_container.add_command(self.controller, Command::ClearRegisters);
                }
                token_container.set_current_end_as_used();
                token_container.update_slice_start();
            }
            token_container.inc_token();
        }

        {
            let logger = self.controller.get_logger_mut();
            logger.builder_end();
            logger.builder_dump_commands(command_container.get_commands());
        }

        Ok(())
    }
}
