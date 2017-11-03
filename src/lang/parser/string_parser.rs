
use super::token::Token;
use super::token_container::TokenContainer;
use super::char_container::CharContainer;
use super::parser_container::ParserContainer;
use super::sub_parser::SubParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;

pub enum StringParserState {
    Nothing,
    Basic,
    Escape,
}

pub struct StringParser {
    state: StringParserState,
}

impl StringParser {
    pub fn new() -> StringParser {
        StringParser { state: StringParserState::Nothing }
    }
}

impl<T> SubParser<T> for StringParser
where
    T: Logger,
{
    fn check(&self, c: char) -> bool {
        match c {
            '"' => true,
            _ => false,
        }
    }

    fn identify(&self) -> &str {
        "String Parser"
    }

    fn reset(&mut self) {
        self.state = StringParserState::Nothing;
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
                StringParserState::Nothing => {
                    match c {
                        '"' => {
                            parser_data.inc_char();
                            StringParserState::Basic
                        }
                        _ => return Err(Error::CheckMismatch(c, ci, li)),
                    }
                }
                StringParserState::Basic => {
                    match c {
                        '\\' => {
                            parser_data.inc_char();
                            StringParserState::Escape
                        }
                        '"' => {
                            parser_data.inc_char();
                            token_container.add_token(
                                controller,
                                Token::String(char_container.flush()),
                            );
                            return Ok(false);
                        }
                        _ => {
                            char_container.add_char(c);
                            parser_data.inc_char();
                            StringParserState::Basic
                        }
                    }
                }
                StringParserState::Escape => {
                    match c {
                        'n' => {
                            char_container.add_char('\n');
                            parser_data.inc_char();
                            StringParserState::Basic
                        }
                        '\\' => {
                            char_container.add_char('\\');
                            parser_data.inc_char();
                            StringParserState::Basic
                        }
                        _ => StringParserState::Basic,
                    }
                }
            };
        }
        Err(Error::ImpossibleState)
    }
}
