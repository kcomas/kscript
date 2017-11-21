
use super::token::Token;
use super::token_container::TokenContainer;
use super::char_container::CharContainer;
use super::parser_container::ParserContainer;
use super::sub_parser::SubParser;
use super::ref_parser::RefParser;
use super::var_parser::VarParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;
use super::util::{do_parse_single, do_parse, top_level_parsers};

pub enum FunctionParserState {
    Nothing,
    MabeArguments,
    DefArguments,
    LoadArguments,
    LoadBody,
}

pub struct FunctionParser {
    state: FunctionParserState,
}

impl FunctionParser {
    pub fn new() -> FunctionParser {
        FunctionParser { state: FunctionParserState::Nothing }
    }
}

impl<T> SubParser<T> for FunctionParser
where
    T: Logger,
{
    fn check(&self, c: char) -> bool {
        match c {
            '{' => true,
            _ => false,
        }
    }

    fn identify(&self) -> &str {
        "Function Parser"
    }

    fn reset(&mut self) {
        self.state = FunctionParserState::Nothing;
    }

    fn parse(
        &mut self,
        controller: &mut Controller<T>,
        parser_data: &mut ParserContainer,
        char_container: &mut CharContainer,
        token_container: &mut TokenContainer,
    ) -> Result<bool, Error> {
        let mut arg_tokens: Vec<Token> = Vec::new();
        let mut statement_tokens: Vec<Token> = Vec::new();
        let mut arg_parsers: [Box<SubParser<T>>; 2] =
            [Box::new(RefParser::new()), Box::new(VarParser::new())];
        let mut done = false;

        {
            let mut arg_container = TokenContainer::new(&mut arg_tokens);
            let mut statement_container = TokenContainer::new(&mut statement_tokens);
            while !parser_data.is_done() {
                let (c, ci, li) = parser_data.get_as_tuple();
                {
                    controller.get_logger_mut().parser_next_char(c, ci, li);
                }
                self.state = match self.state {
                    FunctionParserState::Nothing => {
                        match c {
                            '{' => {
                                parser_data.inc_char();
                                FunctionParserState::MabeArguments
                            }
                            _ => return Err(Error::CheckMismatch(c, ci, li)),
                        }
                    }
                    FunctionParserState::MabeArguments => {
                        match c {
                            ' ' | '\n' => {
                                parser_data.inc_char();
                                FunctionParserState::MabeArguments
                            }
                            '|' => {
                                parser_data.inc_char();
                                FunctionParserState::LoadArguments
                            }
                            _ => return Err(Error::InvalidFunctionArguments(c, ci, li)),
                        }
                    }
                    FunctionParserState::DefArguments => {
                        match c {
                            ' ' | '\n' => {
                                parser_data.inc_char();
                                FunctionParserState::DefArguments
                            }
                            ',' => {
                                parser_data.inc_char();
                                FunctionParserState::LoadArguments
                            }
                            '|' => {
                                parser_data.inc_char();
                                FunctionParserState::LoadBody
                            }
                            _ => return Err(Error::InvalidFunctionArguments(c, ci, li)),
                        }
                    }
                    FunctionParserState::LoadArguments => {
                        let (_exit, used) = do_parse_single(
                            c,
                            parser_data,
                            controller,
                            char_container,
                            &mut arg_container,
                            &mut arg_parsers,
                        )?;

                        match used {
                            true => FunctionParserState::DefArguments,
                            false => {
                                match parser_data.get_current_char() {
                                    '|' => FunctionParserState::DefArguments,
                                    _ => {
                                        parser_data.inc_char();
                                        FunctionParserState::LoadArguments
                                    }
                                }
                            }
                        }
                    }
                    FunctionParserState::LoadBody => {
                        let mut parsers = top_level_parsers();

                        do_parse(
                            parser_data,
                            controller,
                            char_container,
                            &mut statement_container,
                            &mut parsers,
                        )?;

                        if statement_container.len() == 0 {
                            return Err(Error::InvalidFunctionBody(c, ci, li));
                        }
                        done = true;
                        break;
                    }
                };
            }
        }
        if done {
            token_container.add_token(controller, Token::Function(arg_tokens, statement_tokens));
            return Ok(false);

        }
        Err(Error::ImpossibleState)
    }
}
