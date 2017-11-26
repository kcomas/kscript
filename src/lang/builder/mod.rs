
pub mod command;
pub mod command_container;
mod sub_builder;
mod single_command_builder;
mod double_command_builder;
mod io_builder;
mod add_sub_builder;
mod mul_div_mod_builder;
mod exponent_builder;
mod if_builder;
mod loop_builder;
mod util;

use super::controller::Controller;
use super::logger::Logger;
use super::error::Error;
use super::parser::token_container::TokenContainer;
use self::command::Command;
use self::util::{create_new_command_container, top_level_builders};

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

    pub fn run(
        &mut self,
        token_container: &mut TokenContainer,
        commands: &mut Vec<Command>,
    ) -> Result<(), Error> {
        {
            self.controller.get_logger_mut().builder_start();
        }

        let mut builders = top_level_builders();

        let command_container = create_new_command_container(
            self.controller,
            token_container,
            &mut builders,
            commands,
        )?;

        {
            let logger = self.controller.get_logger_mut();
            logger.builder_end();
            logger.builder_dump_commands(command_container.get_commands());
        }

        Ok(())
    }
}
