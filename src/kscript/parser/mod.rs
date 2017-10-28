
pub mod token;
pub mod parser_container;
pub mod token_container;
pub mod char_container;
mod sub_parser;
mod end_parser;
mod var_parser;
mod operator_parser;
mod number_parser;

use super::controller::Controller;
use super::logger::Logger;
use self::token_container::TokenContainer;
use self::char_container::CharContainer;
use self::sub_parser::SubParser;
use self::end_parser::EndParser;
use self::var_parser::VarParser;
use self::number_parser::NumberParser;
use self::operator_parser::OperatorParser;
use self::parser_container::ParserContainer;
use super::error::Error;

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

        let mut token_container = TokenContainer::new();

        let mut parser_data = ParserContainer::new(text_str);

        let mut parsers: [Box<SubParser<T>>; 4] = [
            Box::new(EndParser::new()),
            Box::new(VarParser::new()),
            Box::new(OperatorParser::new()),
            Box::new(NumberParser::new()),
        ];

        while !parser_data.is_done() {
            let mut used = false;
            let (c, ci, li) = parser_data.get_as_tuple();
            {
                self.controller.get_logger_mut().parser_next_char(c, ci, li);
            }

            for i in 0..4 {
                if parsers[i].check(c) {
                    // use parser
                    {
                        self.controller.get_logger_mut().parser_in_parser(
                            parsers[i].identify(),
                        );
                    }
                    let rst = parsers[i].parse(
                        self.controller,
                        &mut parser_data,
                        &mut self.char_container,
                        &mut token_container,
                    );

                    if let Err(kerror) = rst {
                        return Err(kerror);
                    }
                    {
                        self.controller.get_logger_mut().parser_out_parser(
                            parsers[i].identify(),
                        );
                    }
                    used = true;
                    break;
                }
            }
            if !used {
                parser_data.inc_char();
            }
        }

        {
            self.controller.get_logger_mut().parser_end();
        }

        Ok(token_container)
    }
}
