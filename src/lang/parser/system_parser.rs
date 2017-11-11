
use super::token::{Token, SystemCommand};
use super::token_container::TokenContainer;
use super::parser_container::ParserContainer;
use super::char_container::CharContainer;
use super::sub_parser::SubParser;
use super::number_parser::NumberParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;
use super::util::do_parse_single;

pub enum SystemParserState {
    Nothing,
    IsCommand,
    ExitCommand,
}

pub struct SystemParser {
    state: SystemParserState,
    command_container: TokenContainer,
}

impl SystemParser {
    pub fn new() -> SystemParser {
        SystemParser {
            state: SystemParserState::Nothing,
            command_container: TokenContainer::new(),
        }
    }
}

impl<T> SubParser<T> for SystemParser
where
    T: Logger,
{
    fn check(&self, c: char) -> bool {
        match c {
            '\\' => true,
            _ => false,
        }
    }

    fn identify(&self) -> &str {
        "System Parser"
    }

    fn reset(&mut self) {
        self.state = SystemParserState::Nothing;
        self.command_container.clear();
    }

    fn parse(
        &mut self,
        controller: &mut Controller<T>,
        parser_data: &mut ParserContainer,
        char_container: &mut CharContainer,
        token_container: &mut TokenContainer,
    ) -> Result<bool, Error> {
        while !parser_data.is_done() {
            let (c, ci, li) = parser_data.get_as_tuple();
            {
                controller.get_logger_mut().parser_next_char(c, ci, li);
            }
            self.state = match self.state {
                SystemParserState::Nothing => {
                    match c {
                        '\\' => {
                            parser_data.inc_char();
                            SystemParserState::IsCommand
                        }
                        _ => return Err(Error::CheckMismatch(c, ci, li)),
                    }
                }
                SystemParserState::IsCommand => {
                    match c {
                        '\\' => {
                            parser_data.inc_char();
                            SystemParserState::ExitCommand
                        }
                        _ => return Err(Error::InvalidSystemCommand(c, ci, li)),
                    }
                }
                SystemParserState::ExitCommand => {
                    let mut number_parser: [Box<SubParser<T>>; 1] = [Box::new(NumberParser::new())];

                    let (_exit, used) = do_parse_single(
                        c,
                        parser_data,
                        controller,
                        1,
                        &mut number_parser,
                        char_container,
                        &mut self.command_container,
                    )?;

                    match used {
                        true => {
                            if self.command_container.len() != 1 {
                                return Err(Error::InvalidSystemCommand(c, ci, li));
                            }
                            if let Token::Integer(int) = *self.command_container.get(0).unwrap() {
                                let token = Token::System(SystemCommand::Exit(int as u32));
                                token_container.add_token(controller, token);
                                return Ok(false);
                            }
                            return Err(Error::InvalidSystemCommand(c, ci, li));
                        }
                        false => {
                            parser_data.inc_char();
                            SystemParserState::ExitCommand
                        }
                    }
                }
            };
        }
        Err(Error::ImpossibleState)
    }
}
