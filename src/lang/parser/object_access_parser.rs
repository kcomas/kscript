
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
    var_name: Token,
    access_container: TokenContainer,
}

impl ObjectAccessParser {
    pub fn new(var_name: Token) -> ObjectAccessParser {
        ObjectAccessParser {
            state: ObjectAccessParserState::Nothing,
            var_name: var_name,
            access_container: TokenContainer::new(),
        }
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
        self.access_container.clear();
        self.var_name = Token::End;
    }

    fn parse(
        &mut self,
        controller: &mut Controller<T>,
        parser_data: &mut ParserContainer,
        char_container: &mut CharContainer,
        token_container: &mut TokenContainer,
    ) -> Result<bool, Error> {
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
                        &mut self.access_container,
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
                            if self.access_container.len() != 1 {
                                return Err(Error::InvalidObjectAccess(c, ci, li));
                            }

                            let token = self.access_container.get(0).unwrap().clone();
                            token_container.add_token(
                                controller,
                                Token::ObjectAccess(
                                    Box::new(self.var_name.clone()),
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
