
mod controller;
pub mod logger;
pub mod parser;
mod error;
mod util;

use self::controller::Controller;
use self::logger::Logger;
use self::parser::ParserRunner;
use self::parser::token_container::TokenContainer;
use self::error::Error;

#[derive(Debug)]
pub struct Kscript<T: Logger> {
    controller: Controller<T>,
    token_container: Option<TokenContainer>,
}

impl<T> Kscript<T>
where
    T: Logger,
{
    pub fn new(logger: T) -> Kscript<T> {
        Kscript {
            controller: Controller::new(logger),
            token_container: None,
        }
    }

    pub fn get_token_container(&self) -> Option<&TokenContainer> {
        match self.token_container {
            Some(ref container) => Some(container),
            None => None,
        }
    }

    pub fn run(&mut self, text_str: &str) -> Result<(), Error> {
        let mut parser_runner = ParserRunner::new(&mut self.controller);
        self.token_container = None;
        self.token_container = Some(parser_runner.run(text_str)?);
        Ok(())
    }
}
