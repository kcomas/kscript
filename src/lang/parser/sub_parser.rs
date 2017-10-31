
use super::token_container::TokenContainer;
use super::char_container::CharContainer;
use super::parser_container::ParserContainer;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;

pub trait SubParser<T: Logger> {
    // if the current chars can be taken into this parser
    fn check(&self, c: char) -> bool;

    fn identify(&self) -> &str;

    fn reset(&mut self) {}

    fn parse(
        &mut self,
        controller: &mut Controller<T>,
        parser_data: &mut ParserContainer,
        char_container: &mut CharContainer,
        token_container: &mut TokenContainer,
        // if we exit the parent loop
    ) -> Result<bool, Error>;
}
