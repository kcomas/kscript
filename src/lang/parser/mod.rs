
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
mod util;

use super::controller::Controller;
use super::logger::Logger;
use self::token_container::TokenContainer;
use self::parser_container::ParserContainer;
use self::char_container::CharContainer;
use self::sub_parser::SubParser;
use self::end_parser::EndParser;
use self::var_parser::VarParser;
use self::number_parser::NumberParser;
use self::math_parser::MathParser;
use self::operator_parser::OperatorParser;
use self::io_parser::IoParser;
use self::comment_parser::CommentParser;
use self::file_parser::FileParser;
use super::error::Error;
use self::util::do_parse;

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

        let mut parsers: [Box<SubParser<T>>; 8] = [
            Box::new(EndParser::new()),
            Box::new(VarParser::new()),
            Box::new(OperatorParser::new()),
            Box::new(NumberParser::new()),
            Box::new(MathParser::new()),
            Box::new(IoParser::new()),
            Box::new(CommentParser::new()),
            Box::new(FileParser::new()),
        ];

        let mut token_container = TokenContainer::new();

        let mut parser_data = ParserContainer::new(text_str);

        if let Err(kerror) = do_parse(
            &mut parser_data,
            self.controller,
            8,
            &mut parsers,
            &mut self.char_container,
            &mut token_container,
        )
        {
            return Err(kerror);
        }

        {
            self.controller.get_logger_mut().parser_end();
        }

        println!("{:#?}", token_container.get_tokens());

        Ok(token_container)
    }
}
