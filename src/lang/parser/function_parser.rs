
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
    arg_container: TokenContainer,
    statements: TokenContainer,
}

impl FunctionParser {
    pub fn new() -> FunctionParser {
        FunctionParser {
            state: FunctionParserState::Nothing,
            arg_container: TokenContainer::new(),
            statements: TokenContainer::new(),
        }
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
        self.arg_container.clear();
        self.statements.clear();
    }

    fn parse(
        &mut self,
        controller: &mut Controller<T>,
        parser_data: &mut ParserContainer,
        char_container: &mut CharContainer,
        token_container: &mut TokenContainer,
    ) -> Result<bool, Error> {
        let mut arg_parsers: [Box<SubParser<T>>; 2] =
            [Box::new(RefParser::new()), Box::new(VarParser::new_arg())];


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
                        2,
                        &mut arg_parsers,
                        char_container,
                        &mut self.arg_container,
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
                    let (mut parsers, num_parsers) = top_level_parsers();

                    do_parse(
                        parser_data,
                        controller,
                        num_parsers,
                        &mut parsers,
                        char_container,
                        &mut self.statements,
                    )?;

                    if self.statements.len() == 0 {
                        return Err(Error::InvalidFunctionBody(c, ci, li));
                    }

                    token_container.add_token(
                        controller,
                        Token::Function(
                            self.arg_container.get_tokens().clone(),
                            self.statements.get_tokens().clone(),
                        ),
                    );
                    return Ok(false);
                }
            };
        }
        Err(Error::ImpossibleState)
    }
}
