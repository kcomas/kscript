
use super::token::Token;
use super::token_container::TokenContainer;
use super::parser_container::ParserContainer;
use super::char_container::CharContainer;
use super::sub_parser::SubParser;
use super::function_call_parser::FunctionCallParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;
use super::util::do_parse_single;

pub enum VarParserState {
    Nothing,
    Variable,
    Constant,
    FunctionCall,
}

pub struct VarParser {
    state: VarParserState,
    load_calls: bool,
}

impl VarParser {
    pub fn new() -> VarParser {
        VarParser {
            state: VarParserState::Nothing,
            load_calls: true,
        }
    }

    pub fn new_arg() -> VarParser {
        VarParser {
            state: VarParserState::Nothing,
            load_calls: false,
        }
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

    fn identify(&self) -> &str {
        "Var Parser"
    }

    fn reset(&mut self) {
        self.state = VarParserState::Nothing;
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
                VarParserState::Nothing => {
                    match c {
                        '0'...'9' | 'a'...'z' => {
                            char_container.add_char(c);
                            parser_data.inc_char();
                            VarParserState::Variable
                        }
                        'A'...'Z' => {
                            char_container.add_char(c);
                            parser_data.inc_char();
                            VarParserState::Constant
                        }
                        _ => return Err(Error::CheckMismatch(c, ci, li)),
                    }
                }
                VarParserState::Variable => {
                    match c {
                        '0'...'9' | 'a'...'z' => {
                            char_container.add_char(c);
                            parser_data.inc_char();
                            VarParserState::Variable
                        }
                        'A'...'Z' => return Err(Error::InvalidVariableChar(c, ci, li)),
                        '|' => {
                            match self.load_calls {
                                true => VarParserState::FunctionCall,
                                false => {
                                    let token = Token::Var(char_container.flush());
                                    token_container.add_token(controller, token);
                                    return Ok(false);
                                }
                            }
                        }
                        _ => {
                            let token = Token::Var(char_container.flush());
                            token_container.add_token(controller, token);
                            return Ok(false);
                        }
                    }
                }
                VarParserState::Constant => {
                    match c {
                        '0'...'9' | 'A'...'Z' => {
                            char_container.add_char(c);
                            parser_data.inc_char();
                            VarParserState::Constant
                        }
                        'a'...'z' => return Err(Error::InvalidConstantChar(c, ci, li)),
                        '|' => {
                            match self.load_calls {
                                true => VarParserState::FunctionCall,
                                false => {
                                    let token = Token::Const(char_container.flush());
                                    token_container.add_token(controller, token);
                                    return Ok(false);
                                }
                            }
                        }
                        _ => {
                            let token = Token::Const(char_container.flush());
                            token_container.add_token(controller, token);
                            return Ok(false);
                        }
                    }
                }
                VarParserState::FunctionCall => {
                    let mut call_parser: [Box<SubParser<T>>; 1] =
                        [Box::new(FunctionCallParser::new())];

                    let (_exit, _used) = do_parse_single(
                        c,
                        parser_data,
                        controller,
                        1,
                        &mut call_parser,
                        char_container,
                        token_container,
                    )?;

                    return Ok(false);
                }
            };
        }
        Err(Error::ImpossibleState)
    }
}
