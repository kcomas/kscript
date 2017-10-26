
mod controller;
pub mod logger;
mod parser;

use self::controller::Controller;
use self::logger::Logger;
use self::parser::ParserRunner;

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

    pub fn run(&mut self, text_str: &str) {
        let mut parser_runner = ParserRunner::new(&mut self.controller);
        parser_runner.run(text_str);
    }
}
