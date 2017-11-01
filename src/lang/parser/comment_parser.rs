
use super::token::Token;
use super::token_container::TokenContainer;
use super::char_container::CharContainer;
use super::parser_container::ParserContainer;
use super::sub_parser::SubParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;

pub struct CommentParser {}

impl CommentParser {
    pub fn new() -> CommentParser {
        CommentParser {}
    }
}

impl<T> SubParser<T> for CommentParser
where
    T: Logger,
{
    fn check(&self, c: char) -> bool {
        match c {
            '#' => true,
            _ => false,
        }
    }

    fn identify(&self) -> &str {
        "Comment Parser"
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
            match c {
                '#' => parser_data.inc_char(),
                '\n' | '\0' => {
                    parser_data.inc_char();
                    token_container.add_token(controller, Token::Comment(char_container.flush()));
                    return Ok(false);
                }
                _ => {
                    char_container.add_char(c);
                    parser_data.inc_char();
                }
            };
        }
        Err(Error::ImpossibleState)
    }
}
