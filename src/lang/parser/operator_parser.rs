
use super::token::Token;
use super::token_container::TokenContainer;
use super::parser_container::ParserContainer;
use super::char_container::CharContainer;
use super::sub_parser::SubParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;

pub enum OperatorParserState {
    Nothing,
    Equal,
}

pub struct OperatorParser {
    state: OperatorParserState,
}

impl OperatorParser {
    pub fn new() -> OperatorParser {
        OperatorParser { state: OperatorParserState::Nothing }
    }
}

impl<T> SubParser<T> for OperatorParser
where
    T: Logger,
{
    fn check(&self, c: char) -> bool {
        match c {
            '=' | '>' | '<' | '^' | '&' | '!' | '*' => true,
            _ => false,
        }
    }

    fn identify(&self) -> &str {
        "Operator Parser"
    }

    fn reset(&mut self) {
        self.state = OperatorParserState::Nothing;
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
                OperatorParserState::Nothing => {
                    match c {
                        '=' => {
                            parser_data.inc_char();
                            OperatorParserState::Equal
                        }
                        '>' => {
                            parser_data.inc_char();
                            token_container.add_token(controller, Token::Greater);
                            return Ok(false);
                        }
                        '<' => {
                            parser_data.inc_char();
                            token_container.add_token(controller, Token::Less);
                            return Ok(false);
                        }
                        '^' => {
                            parser_data.inc_char();
                            token_container.add_token(controller, Token::Or);
                            return Ok(false);
                        }
                        '&' => {
                            parser_data.inc_char();
                            token_container.add_token(controller, Token::And);
                            return Ok(false);
                        }
                        '!' => {
                            parser_data.inc_char();
                            token_container.add_token(controller, Token::Run);
                            return Ok(false);
                        }
                        '*' => {
                            parser_data.inc_char();
                            token_container.add_token(controller, Token::Dereference);
                            return Ok(false);
                        }
                        _ => return Err(Error::CheckMismatch(c, ci, li)),
                    }
                }
                OperatorParserState::Equal => {
                    match c {
                        '=' => {
                            parser_data.inc_char();
                            token_container.add_token(controller, Token::Equals);
                            return Ok(false);
                        }
                        '!' => {
                            parser_data.inc_char();
                            token_container.add_token(controller, Token::NotEquals);
                            return Ok(false);
                        }
                        '>' => {
                            parser_data.inc_char();
                            token_container.add_token(controller, Token::EqualOrGreater);
                            return Ok(false);
                        }
                        '<' => {
                            parser_data.inc_char();
                            token_container.add_token(controller, Token::EqualOrLess);
                            return Ok(false);
                        }
                        '&' => {
                            parser_data.inc_char();
                            token_container.add_token(controller, Token::TakeReference);
                            return Ok((false));
                        }
                        _ => {
                            token_container.add_token(controller, Token::Assign);
                            return Ok(false);
                        }
                    }
                }
            }
        }
        Err(Error::ImpossibleState)
    }
}
