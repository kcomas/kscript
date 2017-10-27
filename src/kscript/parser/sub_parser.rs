
use super::token::Token;
use super::parser_container::ParserContainer;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;

pub trait SubParser {
    fn new() -> Self;

    // if the current chars can be taken into this parser
    fn check(&self, c: char) -> bool;

    fn parse<T: Logger>(
        &mut self,
        controller: &mut Controller<T>,
        parser_data: &mut ParserContainer,
        current_chars: &mut Vec<char>,
        tokens: &mut Vec<Token>,
    ) -> Result<(), Error>;
}
