
pub mod token;
pub mod parser_container;
pub mod token_container;
pub mod char_container;
mod sub_parser;
mod end_parser;
mod line_end_parser;
mod var_parser;
mod operator_parser;
mod number_parser;
mod math_parser;
mod math_operator_parser;
mod io_parser;
mod comment_parser;
mod file_parser;
mod string_parser;
mod array_parser;
mod dict_parser;
mod bool_parser;
mod cond_builder;
mod conditional_parser;
mod block_end_parser;
mod loop_parser;
mod ref_parser;
mod function_parser;
mod system_parser;
mod function_call_parser;
mod util;

use super::controller::Controller;
use super::logger::Logger;
use self::token_container::TokenContainer;
use self::parser_container::ParserContainer;
use self::char_container::CharContainer;
use super::error::Error;
use self::util::{do_parse, top_level_parsers};

pub struct ParserRunner<'a, T: Logger + 'a> {
    controller: &'a mut Controller<T>,
    char_container: CharContainer,
}

impl<'a, T> ParserRunner<'a, T>
where
    T: Logger + 'a,
{
    pub fn new(controller: &'a mut Controller<T>) -> ParserRunner<'a, T> {
        ParserRunner {
            controller: controller,
            char_container: CharContainer::new(),
        }
    }

    pub fn run(&mut self, text_str: &str) -> Result<TokenContainer, Error> {
        {
            self.controller.get_logger_mut().parser_start();
        }

        let (mut parsers, total_parsers) = top_level_parsers();

        let mut token_container = TokenContainer::new();

        let mut parser_data = ParserContainer::new(text_str);

        do_parse(
            &mut parser_data,
            self.controller,
            total_parsers,
            &mut parsers,
            &mut self.char_container,
            &mut token_container,
        )?;

        {
            self.controller.get_logger_mut().parser_end();
        }

        println!("{:#?}", token_container.get_tokens());

        Ok(token_container)
    }
}
