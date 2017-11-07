
use super::token::Token;
use super::token_container::TokenContainer;
use super::char_container::CharContainer;
use super::parser_container::ParserContainer;
use super::sub_parser::SubParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;

pub enum BoolState {
    Nothing,
    HasTrue,
    HasFalse,
}

pub struct BoolParser {
    state: BoolState,
}

impl BoolParser {
    pub fn new() -> BoolParser {
        BoolParser { state: BoolState::Nothing }
    }
}

impl<T> SubParser<T> for BoolParser
where
    T: Logger,
{
    fn check(&self, c: char) -> bool {
        match c {
            't' | 'f' | 'T' | 'F' => true,
            _ => false,
        }
    }

    fn identify(&self) -> &str {
        "Bool Parser"
    }

    fn reset(&mut self) {
        self.state = BoolState::Nothing;
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
                BoolState::Nothing => {
                    match c {
                        'T' | 't' => {
                            char_container.add_char(c);
                            parser_data.inc_char();
                            BoolState::HasTrue
                        }
                        'F' | 'f' => {
                            char_container.add_char(c);
                            parser_data.inc_char();
                            BoolState::HasFalse
                        }
                        _ => return Err(Error::CheckMismatch(c, ci, li)),
                    }
                }
                BoolState::HasTrue => {
                    match c {
                        'a'...'z' | 'A'...'Z' => return Ok(false),
                        _ => {
                            char_container.flush();
                            token_container.add_token(controller, Token::Bool(true));
                            return Ok(false);
                        }
                    }
                }
                BoolState::HasFalse => {
                    match c {
                        'a'...'z' | 'A'...'Z' => return Ok(false),
                        _ => {
                            char_container.flush();
                            token_container.add_token(controller, Token::Bool(false));
                            return Ok(false);
                        }
                    }
                }
            };
        }
        Err(Error::ImpossibleState)
    }
}
