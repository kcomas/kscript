
use super::token::Token;
use super::token_container::TokenContainer;
use super::char_container::CharContainer;
use super::parser_container::ParserContainer;
use super::sub_parser::SubParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;

pub struct EndParser {}

impl EndParser {
    pub fn new() -> EndParser {
        EndParser {}
    }
}

impl<T> SubParser<T> for EndParser
where
    T: Logger,
{
    fn check(&self, c: char) -> bool {
        match c {
            ';' | '\n' => true,
            _ => false,
        }
    }

    fn identify(&self) -> &str {
        "End Parser"
    }

    fn parse(
        &mut self,
        controller: &mut Controller<T>,
        parser_data: &mut ParserContainer,
        _char_container: &mut CharContainer,
        token_container: &mut TokenContainer,
    ) -> Result<(), Error> {
        match parser_data.get_current_char() {
            ';' => {
                token_container.add_token(controller, Token::End);
                parser_data.inc_char();
            }
            '\n' => {
                token_container.add_token(controller, Token::End);
                parser_data.inc_line();
                parser_data.inc_char();
            }
            _ => {
                let (c, ci, li) = parser_data.get_as_tuple();
                return Err(Error::CheckMismatch(c, ci, li));
            }
        };
        Ok(())
    }
}
