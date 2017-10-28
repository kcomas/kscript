
use super::token::Token;
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

    fn parse(
        &mut self,
        controller: &mut Controller<T>,
        parser_data: &mut ParserContainer,
        current_chars: &mut Vec<char>,
        tokens: &mut Vec<Token>,
    ) -> Result<(), Error> {
        match parser_data.get_current_char() {
            ';' => {
                let token = Token::End;
                {
                    controller.get_logger_mut().parser_add_token(token.clone());
                }
                tokens.push(token);
                parser_data.inc_char();
            }
            '\n' => {
                let token = Token::End;
                {
                    controller.get_logger_mut().parser_add_token(token.clone());
                }
                tokens.push(token);
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
