
use super::token::Token;
use super::token_container::TokenContainer;
use super::char_container::CharContainer;
use super::parser_container::ParserContainer;
use super::sub_parser::SubParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;

pub enum FileParserState {
    Nothing,
    FileString,
}

pub struct FileParser {
    state: FileParserState,
}

impl FileParser {
    pub fn new() -> FileParser {
        FileParser { state: FileParserState::Nothing }
    }
}

impl<T> SubParser<T> for FileParser
where
    T: Logger,
{
    fn check(&self, c: char) -> bool {
        match c {
            '\'' => true,
            _ => false,
        }
    }

    fn identify(&self) -> &str {
        "File Parser"
    }

    fn reset(&mut self) {
        self.state = FileParserState::Nothing;
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
                FileParserState::Nothing => {
                    match c {
                        '\'' => {
                            parser_data.inc_char();
                            FileParserState::FileString
                        }
                        _ => return Err(Error::CheckMismatch(c, ci, li)),
                    }
                }
                FileParserState::FileString => {
                    match c {
                        '\'' => {
                            parser_data.inc_char();
                            token_container.add_token(
                                controller,
                                Token::File(char_container.flush()),
                            );
                            return Ok(false);
                        }
                        _ => {
                            char_container.add_char(c);
                            parser_data.inc_char();
                            FileParserState::FileString
                        }
                    }
                }
            };
        }
        Err(Error::ImpossibleState)
    }
}
