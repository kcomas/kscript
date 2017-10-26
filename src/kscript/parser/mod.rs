
mod token;

use super::controller::Controller;
use super::logger::Logger;
use self::token::Token;

#[derive(Debug)]
pub struct ParserRunner<'a, T: Logger + 'a> {
    controller: &'a mut Controller<T>,
    tokens: Vec<Vec<Token>>,
}

impl<'a, T> ParserRunner<'a, T>
where
    T: Logger + 'a,
{
    pub fn new(controller: &'a mut Controller<T>) -> ParserRunner<'a, T> {
        ParserRunner {
            controller: controller,
            tokens: Vec::new(),
        }
    }

    pub fn run(&mut self, text_str: &str) {
        {
            self.controller.get_logger_mut().parser_start();
        }
        let text_vec: Vec<char> = text_str.chars().collect();
        let current_char = 0;
        let current_line = 0;

    }
}
