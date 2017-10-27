
pub mod token;
pub mod parser_container;
mod sub_parser;
mod end_parser;

use super::controller::Controller;
use super::logger::Logger;
use self::token::Token;
use self::end_parser::EndParser;
use self::parser_container::ParserContainer;

#[derive(Debug)]
pub struct ParserRunner<'a, T: Logger + 'a> {
    controller: &'a mut Controller<T>,
    current_chars: Vec<char>,
    tokens: Vec<Token>,
}

impl<'a, T> ParserRunner<'a, T>
where
    T: Logger + 'a,
{
    pub fn new(controller: &'a mut Controller<T>) -> ParserRunner<'a, T> {
        ParserRunner {
            controller: controller,
            current_chars: Vec::new(),
            tokens: Vec::new(),
        }
    }

    pub fn run(&mut self, text_str: &str) {
        {
            self.controller.get_logger_mut().parser_start();
        }

        let mut parser_data = ParserContainer::new(text_str);


    }
}
