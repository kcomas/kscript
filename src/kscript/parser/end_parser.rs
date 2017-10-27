
use super::token::Token;
use super::parser_container::ParserContainer;
use super::sub_parser::SubParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;

#[derive(Debug)]
pub struct EndParser {}

impl SubParser for EndParser {
    fn new() -> EndParser {
        EndParser {}
    }

    fn check(&self, c: char) -> bool {
        match c {
            ';' | '\n' => true,
            _ => false,
        }
    }

    fn parse<T: Logger>(
        &mut self,
        controller: &mut Controller<T>,
        parser_data: &mut ParserContainer,
        current_chars: &mut Vec<char>,
        tokens: &mut Vec<Token>,
    ) -> Result<(), Error> {
        match parser_data.get_current_char() {
            ';' => {
                tokens.push(Token::End);
                parser_data.inc_char();
            }
            '\n' => {
                tokens.push(Token::End);
                parser_data.inc_line();
                parser_data.inc_char();
            }
            _ => {
                let (c, ci, li) = parser_data.get_as_tuple();
                return Err(Error::InvalidEndChar(c, ci, li));
            }
        };
        Ok(())
    }
}
