
use super::token_container::TokenContainer;
use super::parser_container::ParserContainer;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;

pub trait SubParser<T: Logger> {
    // if the current chars can be taken into this parser
    fn check(&self, c: char) -> bool;

    fn parse(
        &mut self,
        controller: &mut Controller<T>,
        parser_data: &mut ParserContainer,
        current_chars: &mut Vec<char>,
        token_container: &mut TokenContainer,
    ) -> Result<(), Error>;
}
