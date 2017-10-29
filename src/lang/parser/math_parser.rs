
use super::token::Token;
use super::token_container::TokenContainer;
use super::parser_container::ParserContainer;
use super::char_container::CharContainer;
use super::sub_parser::SubParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;

pub struct MathParser {}

impl MathParser {
    pub fn new() -> MathParser {
        MathParser {}
    }
}

impl<T> SubParser<T> for MathParser
where
    T: Logger,
{
    fn check(&self, c: char) -> bool {
        match c {
            '(' => true,
            _ => false,
        }
    }

    fn identify(&self) -> String {
        "Math Parser".to_string()
    }

    fn parse(
        &mut self,
        controller: &mut Controller<T>,
        parser_data: &mut ParserContainer,
        char_container: &mut CharContainer,
        token_container: &mut TokenContainer,
    ) -> Result<(), Error> {
        Ok(())
    }
}
