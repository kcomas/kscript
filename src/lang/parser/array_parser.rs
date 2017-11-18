
use super::token::Token;
use super::token_container::TokenContainer;
use super::char_container::CharContainer;
use super::parser_container::ParserContainer;
use super::sub_parser::SubParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;
use super::util::{do_parse_single, object_value_parsers};

pub enum ArrayParserState {
    Nothing,
    IsArray,
    LoadItem,
}

pub struct ArrayParser {
    state: ArrayParserState,
    array_container: TokenContainer,
}

impl ArrayParser {
    pub fn new() -> ArrayParser {
        ArrayParser {
            state: ArrayParserState::Nothing,
            array_container: TokenContainer::new(),
        }
    }
}

impl<T> SubParser<T> for ArrayParser
where
    T: Logger,
{
    fn check(&self, c: char) -> bool {
        match c {
            '@' => true,
            _ => false,
        }
    }

    fn identify(&self) -> &str {
        "Array Parser"
    }

    fn reset(&mut self) {
        self.state = ArrayParserState::Nothing;
        self.array_container.clear();
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
                ArrayParserState::Nothing => {
                    match c {
                        '@' => {
                            parser_data.inc_char();
                            ArrayParserState::IsArray
                        }
                        _ => return Err(Error::CheckMismatch(c, ci, li)),
                    }
                }
                ArrayParserState::IsArray => {
                    match c {
                        '[' => {
                            parser_data.inc_char();
                            ArrayParserState::LoadItem
                        }
                        ',' => {
                            parser_data.inc_char();
                            ArrayParserState::LoadItem
                        }
                        ' ' | '\n' => {
                            parser_data.inc_char();
                            ArrayParserState::IsArray
                        }
                        ']' => {
                            parser_data.inc_char();
                            token_container.add_token(
                                controller,
                                Token::Array(self.array_container.get_tokens().clone()),
                            );
                            return Ok(false);
                        }
                        _ => return Err(Error::InvalidArrayOp(c, ci, li)),
                    }
                }
                ArrayParserState::LoadItem => {
                    let (_exit, used) = do_parse_single(
                        c,
                        parser_data,
                        controller,
                        char_container,
                        &mut self.array_container,
                        &mut parsers,
                    )?;

                    match used {
                        true => ArrayParserState::IsArray,
                        false => {
                            match c {
                                // empty arr
                                ']' => ArrayParserState::IsArray,
                                _ => {
                                    parser_data.inc_char();
                                    ArrayParserState::LoadItem
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
