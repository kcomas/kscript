
use super::token::Token;
use super::token_container::TokenContainer;
use super::parser_container::ParserContainer;
use super::sub_parser::SubParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;

pub enum VarParserState {
    Nothing,
    Variable,
    Constant,
    Done,
}

pub struct VarParser {
    state: VarParserState,
}

impl VarParser {
    pub fn new() -> VarParser {
        VarParser { state: VarParserState::Nothing }
    }
}

impl<T> SubParser<T> for VarParser
where
    T: Logger,
{
    fn check(&self, c: char) -> bool {
        match c {
            'a'...'z' | 'A'...'Z' => true,
            _ => false,
        }
    }

    fn parse(
        &mut self,
        controller: &mut Controller<T>,
        parser_data: &mut ParserContainer,
        current_chars: &mut Vec<char>,
        token_container: &mut TokenContainer,
    ) -> Result<(), Error> {
        while !parser_data.is_done() {
            let (c, ci, li) = parser_data.get_as_tuple();
            {
                controller.get_logger_mut().parser_next_char(c, ci, li);
            }
            self.state = match self.state {
                VarParserState::Nothing => {
                    match c {
                        'a'...'z' => {
                            current_chars.push(c);
                            parser_data.inc_char();
                            VarParserState::Variable
                        }
                        'A'...'Z' => {
                            current_chars.push(c);
                            parser_data.inc_char();
                            VarParserState::Constant
                        }
                        _ => return Err(Error::CheckMismatch(c, ci, li)),
                    }
                }
                VarParserState::Variable => {
                    match c {
                        'a'...'z' => {
                            current_chars.push(c);
                            parser_data.inc_char();
                            VarParserState::Variable
                        }
                        'A'...'Z' | '0'...'9' => return Err(Error::InvalidVariableChar(c, ci, li)),
                        _ => {
                            let token = Token::Var(current_chars.clone().into_iter().collect());
                            current_chars.clear();
                            token_container.add_token(controller, token);
                            VarParserState::Done
                        }
                    }
                }
                VarParserState::Constant => {
                    match c {
                        'A'...'Z' => {
                            current_chars.push(c);
                            parser_data.inc_char();
                            VarParserState::Constant
                        }
                        'a'...'z' | '0'...'9' => return Err(Error::InvalidConstantChar(c, ci, li)),
                        _ => {
                            let token =
                                Token::Constant(current_chars.clone().into_iter().collect());
                            current_chars.clear();
                            token_container.add_token(controller, token);
                            VarParserState::Done
                        }
                    }
                }
                VarParserState::Done => return Ok(()),
            };
        }
        Err(Error::ImpossibleState)
    }
}
