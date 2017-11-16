
mod controller;
pub mod logger;
pub mod parser;
pub mod builder;
mod error;
mod util;

use self::controller::Controller;
use self::logger::Logger;
use self::parser::ParserRunner;
use self::builder::BuilderRunner;
use self::parser::token_container::TokenContainer;
use self::builder::command_container::CommandContainer;
use self::error::Error;
use self::util::load_file_to_string;

pub struct Kscript<T: Logger> {
    controller: Controller<T>,
    token_container: Option<TokenContainer>,
    command_container: Option<CommandContainer>,
}

impl<T> Kscript<T>
where
    T: Logger,
{
    pub fn new(logger: T) -> Kscript<T> {
        Kscript {
            controller: Controller::new(logger),
            token_container: None,
            command_container: None,
        }
    }

    pub fn get_token_container(&self) -> Option<&TokenContainer> {
        match self.token_container {
            Some(ref container) => Some(container),
            None => None,
        }
    }

    pub fn get_command_container(&self) -> Option<&CommandContainer> {
        match self.command_container {
            Some(ref container) => Some(container),
            None => None,
        }
    }

    pub fn run(&mut self, text_str: &str) -> Result<(), Error> {
        self.run_build_tokens_commands(text_str)?;
        Ok(())
    }

    pub fn run_file(&mut self, file_name: &str) -> Result<(), Error> {
        match load_file_to_string(file_name) {
            Ok(ref file_string) => self.run(file_string),
            Err(file_error) => Err(Error::FileLoadFail(file_error)),
        }
    }

    pub fn run_build_tokens(&mut self, text_str: &str) -> Result<(), Error> {
        let mut parser_runner = ParserRunner::new(&mut self.controller);
        self.token_container = None;
        self.token_container = Some(parser_runner.run(text_str)?);
        Ok(())
    }

    pub fn run_build_tokens_commands(&mut self, text_str: &str) -> Result<(), Error> {
        self.run_build_tokens(text_str)?;
        {
            let mut builder_runner = BuilderRunner::new(&mut self.controller);
            self.command_container = None;
            if let Some(ref mut token_container) = self.token_container {
                self.command_container = Some(builder_runner.run(token_container)?);
            } else {
                return Err(Error::ImpossibleState);
            }
        }
        Ok(())
    }
}
