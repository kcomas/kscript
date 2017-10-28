
pub mod token;
pub mod parser_container;
mod sub_parser;
mod end_parser;
mod var_parser;

use super::controller::Controller;
use super::logger::Logger;
use self::token::Token;
use self::sub_parser::SubParser;
use self::end_parser::EndParser;
use self::var_parser::VarParser;
use self::parser_container::ParserContainer;
use super::error::Error;

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

    pub fn run(&mut self, text_str: &str) -> Result<(), Error> {
        {
            self.controller.get_logger_mut().parser_start();
        }

        let mut parser_data = ParserContainer::new(text_str);

        let mut parsers: [Box<SubParser<T>>; 2] =
            [Box::new(EndParser::new()), Box::new(VarParser::new())];

        while !parser_data.is_done() {
            let mut used = false;
            let (c, ci, li) = parser_data.get_as_tuple();
            {
                self.controller.get_logger_mut().parser_next_char(c, ci, li);
            }

            for i in 0..2 {
                if parsers[i].check(c) {
                    // use parser
                    let rst = parsers[i].parse(
                        self.controller,
                        &mut parser_data,
                        &mut self.current_chars,
                        &mut self.tokens,
                    );

                    if let Err(kerror) = rst {
                        return Err(kerror);
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

        Ok(())
    }
}
