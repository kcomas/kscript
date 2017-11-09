
use super::token_container::TokenContainer;
use super::char_container::CharContainer;
use super::parser_container::ParserContainer;
use super::sub_parser::SubParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;

pub struct BlockEndParser {}

impl BlockEndParser {
    pub fn new() -> BlockEndParser {
        BlockEndParser {}
    }
}

impl<T> SubParser<T> for BlockEndParser
where
    T: Logger,
{
    fn check(&self, c: char) -> bool {
        match c {
            '}' => true,
            _ => false,
        }
    }

    fn identify(&self) -> &str {
        "Block End Parser"
    }

    fn parse(
        &mut self,
        _controller: &mut Controller<T>,
        parser_data: &mut ParserContainer,
        _char_container: &mut CharContainer,
        _token_container: &mut TokenContainer,
    ) -> Result<bool, Error> {
        match parser_data.get_current_char() {
            '}' => {
                parser_data.inc_char();
                Ok(true)
            }
            _ => {
                let (c, ci, li) = parser_data.get_as_tuple();
                Err(Error::CheckMismatch(c, ci, li))
            }
        }
    }
}
