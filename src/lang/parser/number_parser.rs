
use super::token::Token;
use super::token_container::TokenContainer;
use super::parser_container::ParserContainer;
use super::char_container::CharContainer;
use super::sub_parser::SubParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;

pub enum NumberParserState {
    Integer,
    Float,
}

pub struct NumberParser {
    state: NumberParserState,
}

impl NumberParser {
    pub fn new() -> NumberParser {
        NumberParser { state: NumberParserState::Integer }
    }
}

impl<T> SubParser<T> for NumberParser
where
    T: Logger,
{
    fn check(&self, c: char) -> bool {
        match c {
            '0'...'9' => true,
            _ => false,
        }
    }

    fn identify(&self) -> &str {
        "Number Parser"
    }

    fn parse(
        &mut self,
        controller: &mut Controller<T>,
        parser_data: &mut ParserContainer,
        char_container: &mut CharContainer,
        token_container: &mut TokenContainer,
        exit: &mut bool,
    ) -> Result<(), Error> {
        while !parser_data.is_done() {
            let (c, ci, li) = parser_data.get_as_tuple();
            {
                controller.get_logger_mut().parser_next_char(c, ci, li);
            }
            self.state = match self.state {
                NumberParserState::Integer => {
                    match c {
                        '0'...'9' => {
                            char_container.add_char(c);
                            parser_data.inc_char();
                            NumberParserState::Integer
                        }
                        '.' => {
                            char_container.add_char(c);
                            parser_data.inc_char();
                            NumberParserState::Float
                        }
                        _ => {
                            let num = char_container.flush();
                            let token = match num.parse() {
                                Ok(num) => Token::Integer(num),
                                Err(err) => return Err(Error::IntegerParseFail(err)),
                            };
                            token_container.add_token(controller, token);
                            return Ok(());
                        }
                    }
                }
                NumberParserState::Float => {
                    match c {
                        '0'...'9' => {
                            char_container.add_char(c);
                            parser_data.inc_char();
                            NumberParserState::Float
                        }
                        _ => {
                            let num = char_container.flush();
                            let token = match num.parse() {
                                Ok(num) => Token::Float(num),
                                Err(err) => return Err(Error::FloatParseFail(err)),
                            };
                            token_container.add_token(controller, token);
                            return Ok(());
                        }
                    }
                }
            };
        }
        Err(Error::ImpossibleState)
    }
}
