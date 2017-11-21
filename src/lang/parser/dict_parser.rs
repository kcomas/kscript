
use super::token::Token;
use super::token_container::TokenContainer;
use super::char_container::CharContainer;
use super::parser_container::ParserContainer;
use super::sub_parser::SubParser;
use super::string_parser::StringParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;
use super::util::{do_parse_single, object_value_parsers};

pub enum DictParserState {
    Nothing,
    IsDict,
    LoadKey,
    LoadValue,
}

pub struct DictParser {
    state: DictParserState,
}

impl DictParser {
    pub fn new() -> DictParser {
        DictParser { state: DictParserState::Nothing }
    }
}

impl<T> SubParser<T> for DictParser
where
    T: Logger,
{
    fn check(&self, c: char) -> bool {
        match c {
            '%' => true,
            _ => false,
        }
    }

    fn identify(&self) -> &str {
        "Dictionary Parser"
    }

    fn reset(&mut self) {
        self.state = DictParserState::Nothing;
    }

    fn parse(
        &mut self,
        controller: &mut Controller<T>,
        parser_data: &mut ParserContainer,
        char_container: &mut CharContainer,
        token_container: &mut TokenContainer,
    ) -> Result<bool, Error> {
        let mut key_tokens: Vec<Token> = Vec::new();
        let mut value_tokens: Vec<Token> = Vec::new();
        let mut parsers = object_value_parsers();
        let mut string_parser: [Box<SubParser<T>>; 1] = [Box::new(StringParser::new())];
        let mut done = false;

        {
            let mut key_container = TokenContainer::new(&mut key_tokens);
            let mut value_container = TokenContainer::new(&mut value_tokens);
            while !parser_data.is_done() {
                let (c, ci, li) = parser_data.get_as_tuple();
                {
                    controller.get_logger_mut().parser_next_char(c, ci, li);
                }
                self.state = match self.state {
                    DictParserState::Nothing => {
                        match c {
                            '%' => {
                                parser_data.inc_char();
                                DictParserState::IsDict
                            }
                            _ => return Err(Error::CheckMismatch(c, ci, li)),
                        }
                    }
                    DictParserState::IsDict => {
                        match c {
                            '[' => {
                                parser_data.inc_char();
                                DictParserState::LoadKey
                            }
                            ':' => {
                                parser_data.inc_char();
                                DictParserState::LoadValue
                            }
                            ',' => {
                                parser_data.inc_char();
                                DictParserState::LoadKey
                            }
                            ' ' | '\n' => {
                                parser_data.inc_char();
                                DictParserState::IsDict
                            }
                            ']' => {
                                parser_data.inc_char();
                                done = true;
                                break;
                            }
                            _ => return Err(Error::InvaliDictOp(c, ci, li)),
                        }
                    }
                    DictParserState::LoadKey => {
                        let (_exit, used) = do_parse_single(
                            c,
                            parser_data,
                            controller,
                            char_container,
                            &mut key_container,
                            &mut string_parser,
                        )?;

                        match used {
                            true => DictParserState::IsDict,
                            false => {
                                match c {
                                    // empty dict
                                    ']' => DictParserState::IsDict,
                                    _ => {
                                        parser_data.inc_char();
                                        DictParserState::LoadKey
                                    }
                                }
                            }
                        }
                    }
                    DictParserState::LoadValue => {
                        let (_exit, used) = do_parse_single(
                            c,
                            parser_data,
                            controller,
                            char_container,
                            &mut value_container,
                            &mut parsers,
                        )?;

                        match used {
                            true => DictParserState::IsDict,
                            false => {
                                parser_data.inc_char();
                                DictParserState::LoadValue
                            }
                        }
                    }
                };
            }
        }
        if done {
            let dict_token = Token::Dict(key_tokens, value_tokens);
            token_container.add_token(controller, dict_token);
            return Ok(false);
        }
        Err(Error::ImpossibleState)
    }
}
