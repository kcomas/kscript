
use super::token::Token;
use super::token_container::TokenContainer;
use super::char_container::CharContainer;
use super::parser_container::ParserContainer;
use super::sub_parser::SubParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;

pub enum IoParserState {
    Nothing,
    IoIn,
    IoOut,
}

pub struct IoParser {
    state: IoParserState,
}

impl IoParser {
    pub fn new() -> IoParser {
        IoParser { state: IoParserState::Nothing }
    }
}

impl<T> SubParser<T> for IoParser
where
    T: Logger,
{
    fn check(&self, c: char) -> bool {
        match c {
            '>' | '<' => true,
            _ => false,
        }
    }

    fn identify(&self) -> &str {
        "IO Parser"
    }

    fn parse(
        &mut self,
        controller: &mut Controller<T>,
        parser_data: &mut ParserContainer,
        _char_container: &mut CharContainer,
        token_container: &mut TokenContainer,
    ) -> Result<bool, Error> {
        while !parser_data.is_done() {
            let (c, ci, li) = parser_data.get_as_tuple();
            {
                controller.get_logger_mut().parser_next_char(c, ci, li);
            }
            self.state = match self.state {
                IoParserState::Nothing => {
                    match c {
                        '>' => {
                            parser_data.inc_char();
                            IoParserState::IoIn
                        }
                        '<' => {
                            parser_data.inc_char();
                            IoParserState::IoOut
                        }
                        _ => return Err(Error::CheckMismatch(c, ci, li)),
                    }
                }
                IoParserState::IoIn => {
                    match c {
                        '>' => {
                            parser_data.inc_char();
                            token_container.add_token(controller, Token::IoAppend);
                            return Ok(false);
                        }
                        _ => {
                            token_container.add_token(controller, Token::IoWrite);
                            return Ok(false);
                        }
                    }
                }
                IoParserState::IoOut => {
                    match c {
                        '<' => {
                            parser_data.inc_char();
                            token_container.add_token(controller, Token::IoReadAppend);
                            return Ok(false);
                        }
                        _ => {
                            token_container.add_token(controller, Token::IoRead);
                            return Ok(false);
                        }
                    }
                }
            };
        }
        Err(Error::ImpossibleState)
    }
}
