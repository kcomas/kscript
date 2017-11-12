
use super::controller::Controller;
use super::logger::Logger;
use super::error::Error;
use super::parser::token_container::TokenContainer;

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

    pub fn run(&mut self, token_container: &TokenContainer) -> Result<(), Error> {
        {
            self.controller.get_logger_mut().builder_start();
        }

        {
            self.controller.get_logger_mut().builder_end();
        }

        Ok(())
    }
}
