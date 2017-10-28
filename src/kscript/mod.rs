
mod controller;
pub mod logger;
mod parser;
mod error;

use self::controller::Controller;
use self::logger::Logger;
use self::parser::ParserRunner;
use self::error::Error;

#[derive(Debug)]
pub struct Kscript<T: Logger> {
    controller: Controller<T>,
}

impl<T> Kscript<T>
where
    T: Logger,
{
    pub fn new(logger: T) -> Kscript<T> {
        Kscript { controller: Controller::new(logger) }
    }

    pub fn run(&mut self, text_str: &str) -> Result<(), Error> {
        let mut parser_runner = ParserRunner::new(&mut self.controller);
        let tokens = match parser_runner.run(text_str) {
            Ok(tokens) => tokens,
            Err(kerror) => return Err(kerror),
        };
        println!("{:?}", tokens);
        Ok(())
    }
}
