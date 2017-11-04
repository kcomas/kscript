
use super::token::Token;
use super::token_container::TokenContainer;
use super::char_container::CharContainer;
use super::parser_container::ParserContainer;
use super::sub_parser::SubParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;
use super::util::do_parse;

pub enum ArrayParserState {
    Nothing,
}

pub struct ArrayParser {
    state: ArrayParserState,
}

impl<T> SubParser<T> for ArrayParser
where
    T: Logger,
{
    fn check(&self, c: char) -> bool {
        match c {
            '@' => true,
            _ => false,
        }
    }

    fn identify(&self) -> &str {
        "Array Parser"
    }

    fn reset(&mut self) {
        self.state = ArrayParserState::Nothing;
    }

    fn parse(
        &mut self,
        controller: &mut Controller<T>,
        parser_data: &mut ParserContainer,
        char_container: &mut CharContainer,
        token_container: &mut TokenContainer,
    ) -> Result<bool, Error> {

        Err(Error::ImpossibleState)
    }
}
