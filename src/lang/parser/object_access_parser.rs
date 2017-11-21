
use super::token::Token;
use super::token_container::TokenContainer;
use super::char_container::CharContainer;
use super::parser_container::ParserContainer;
use super::sub_parser::SubParser;
use super::string_parser::StringParser;
use super::number_parser::NumberParser;
use super::var_parser::VarParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;
use super::util::do_parse_single;

pub enum ObjectAccessParserState {
    Nothing,
    Access,
    Finish,
}

pub struct ObjectAccessParser {
    state: ObjectAccessParserState,
}

impl ObjectAccessParser {
    pub fn new() -> ObjectAccessParser {
        ObjectAccessParser { state: ObjectAccessParserState::Nothing }
    }
}

impl<T> SubParser<T> for ObjectAccessParser
where
    T: Logger,
{
    fn check(&self, c: char) -> bool {
        match c {
            '[' => true,
            _ => false,
        }
    }

    fn identify(&self) -> &str {
        "Object Access Parser"
    }

    fn reset(&mut self) {
        self.state = ObjectAccessParserState::Nothing;
    }

    fn parse(
        &mut self,
        controller: &mut Controller<T>,
        parser_data: &mut ParserContainer,
        char_container: &mut CharContainer,
        token_container: &mut TokenContainer,
    ) -> Result<bool, Error> {
        let mut access_tokens: Vec<Token> = Vec::new();
        let mut access_container = TokenContainer::new(&mut access_tokens);
        let mut parsers: [Box<SubParser<T>>; 3] = [
            Box::new(NumberParser::new()),
            Box::new(StringParser::new()),
            Box::new(VarParser::new()),
        ];

        while !parser_data.is_done() {
            let (c, ci, li) = parser_data.get_as_tuple();
            {
                controller.get_logger_mut().parser_next_char(c, ci, li);
            }
            self.state = match self.state {
                ObjectAccessParserState::Nothing => {
                    match c {
                        '[' => {
                            parser_data.inc_char();
                            ObjectAccessParserState::Access
                        }
                        _ => return Err(Error::CheckMismatch(c, ci, li)),
                    }
                }
                ObjectAccessParserState::Access => {
                    let (_exit, used) = do_parse_single(
                        c,
                        parser_data,
                        controller,
                        char_container,
                        &mut access_container,
                        &mut parsers,
                    )?;

                    match used {
                        true => ObjectAccessParserState::Finish,
                        false => {
                            parser_data.inc_char();
                            ObjectAccessParserState::Access
                        }
                    }
                }
                ObjectAccessParserState::Finish => {
                    match c {
                        ']' => {
                            parser_data.inc_char();
                            if access_container.len() != 1 {
                                return Err(Error::InvalidObjectAccess(c, ci, li));
                            }

                            let prev_token = match token_container.pop() {
                                Some(prev_t) => prev_t,
                                _ => return Err(Error::InvalidObjectAccess(c, ci, li)),
                            };

                            // @TODO check if this breaks tests
                            let token = access_container.pop().unwrap();
                            token_container.add_token(
                                controller,
                                Token::ObjectAccess(
                                    Box::new(prev_token),
                                    Box::new(token),
                                ),
                            );

                            return Ok(false);
                        }
                        _ => {
                            parser_data.inc_char();
                            ObjectAccessParserState::Finish
                        }
                    }
                }
            };
        }
        Err(Error::ImpossibleState)
    }
}
