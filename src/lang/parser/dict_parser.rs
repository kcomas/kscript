
use super::token::Token;
use super::token_container::TokenContainer;
use super::char_container::CharContainer;
use super::parser_container::ParserContainer;
use super::sub_parser::SubParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;
use super::util::{do_parse_single, object_value_parsers};

pub enum DictParserState {
    Nothing,
    IsDict,
    LoadKey,
    LoadValue,
}

pub struct DictParser {
    state: DictParserState,
    key_container: TokenContainer,
    value_container: TokenContainer,
}

impl DictParser {
    pub fn new() -> DictParser {
        DictParser {
            state: DictParserState::Nothing,
            key_container: TokenContainer::new(),
            value_container: TokenContainer::new(),
        }
    }
}

impl<T> SubParser<T> for DictParser
where
    T: Logger,
{
    fn check(&self, c: char) -> bool {
        match c {
            '%' => true,
            _ => false,
        }
    }

    fn identify(&self) -> &str {
        "Dictionary Parser"
    }

    fn reset(&mut self) {
        self.state = DictParserState::Nothing;
        self.key_container.clear();
        self.value_container.clear();
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
