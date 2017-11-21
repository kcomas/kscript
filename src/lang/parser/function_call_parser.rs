
use super::token::Token;
use super::token_container::TokenContainer;
use super::char_container::CharContainer;
use super::parser_container::ParserContainer;
use super::sub_parser::SubParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;
use super::util::{do_parse_single, object_value_parsers};

pub enum FunctionCallParserState {
    Nothing,
    MabeArguments,
    LoadArguments,
}

pub struct FunctionCallParser {
    state: FunctionCallParserState,
}

impl FunctionCallParser {
    pub fn new() -> FunctionCallParser {
        FunctionCallParser { state: FunctionCallParserState::Nothing }
    }
}

impl<T> SubParser<T> for FunctionCallParser
where
    T: Logger,
{
    fn check(&self, c: char) -> bool {
        match c {
            '|' => true,
            _ => false,
        }
    }

    fn identify(&self) -> &str {
        "Function Call Parser"
    }

    fn reset(&mut self) {
        self.state = FunctionCallParserState::Nothing;
    }

    fn parse(
        &mut self,
        controller: &mut Controller<T>,
        parser_data: &mut ParserContainer,
        char_container: &mut CharContainer,
        token_container: &mut TokenContainer,
    ) -> Result<bool, Error> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut parsers = object_value_parsers();
        let mut done = false;

        {
            let mut arg_container = TokenContainer::new(&mut tokens);
            while !parser_data.is_done() {
                let (c, ci, li) = parser_data.get_as_tuple();
                {
                    controller.get_logger_mut().parser_next_char(c, ci, li);
                }
                self.state = match self.state {
                    FunctionCallParserState::Nothing => {
                        match c {
                            '|' => {
                                parser_data.inc_char();
                                FunctionCallParserState::LoadArguments
                            }
                            _ => return Err(Error::CheckMismatch(c, ci, li)),
                        }
                    }
                    FunctionCallParserState::MabeArguments => {
                        match c {
                            ' ' | '\n' => {
                                parser_data.inc_char();
                                FunctionCallParserState::MabeArguments
                            }
                            ',' => {
                                parser_data.inc_char();
                                FunctionCallParserState::LoadArguments
                            }
                            '|' => {
                                parser_data.inc_char();
                                done = true;
                                break;
                            }
                            _ => return Err(Error::InvalidFunctionCall(c, ci, li)),
                        }
                    }
                    FunctionCallParserState::LoadArguments => {
                        let (_exit, used) = do_parse_single(
                            c,
                            parser_data,
                            controller,
                            char_container,
                            &mut arg_container,
                            &mut parsers,
                        )?;

                        match used {
                            true => FunctionCallParserState::MabeArguments,
                            false => {
                                match parser_data.get_current_char() {
                                    '|' => FunctionCallParserState::MabeArguments,
                                    _ => {
                                        parser_data.inc_char();
                                        FunctionCallParserState::LoadArguments
                                    }
                                }
                            }
                        }
                    }
                };
            }
        }
        if done {
            let (c, ci, li) = parser_data.get_as_tuple();
            let prev_token = match token_container.pop() {
                Some(prev_t) => prev_t,
                _ => return Err(Error::InvalidFunctionCall(c, ci, li)),
            };
            let token = Token::FunctionCall(Box::new(prev_token), tokens);
            token_container.add_token(controller, token);
            return Ok(false);

        }
        Err(Error::ImpossibleState)
    }
}
