
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
    var_name: Token,
    arg_container: TokenContainer,
}

impl FunctionCallParser {
    pub fn new(var_name: Token) -> FunctionCallParser {
        FunctionCallParser {
            state: FunctionCallParserState::Nothing,
            var_name: var_name,
            arg_container: TokenContainer::new(),
        }
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
        self.arg_container.clear();
        self.var_name = Token::End;
    }

    fn parse(
        &mut self,
        controller: &mut Controller<T>,
        parser_data: &mut ParserContainer,
        char_container: &mut CharContainer,
        token_container: &mut TokenContainer,
    ) -> Result<bool, Error> {
        let mut parsers = object_value_parsers();

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
                            let token = Token::FunctionCall(
                                Box::new(self.var_name.clone()),
                                self.arg_container.get_tokens().clone(),
                            );
                            token_container.add_token(controller, token);
                            return Ok(false);
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
                        &mut self.arg_container,
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
        Err(Error::ImpossibleState)
    }
}
