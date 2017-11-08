
use super::token::Token;
use super::token_container::TokenContainer;
use super::char_container::CharContainer;
use super::parser_container::ParserContainer;
use super::sub_parser::SubParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;
use super::util::{do_parse_single, loop_conditional_parsers};

pub enum ConditionalState {
    Nothing,
    ItemA,
    Conditional,
    ItemB,
    MabeBlocks,
    TrueStatements,
    FalseStatements,
}

pub struct ConditionalParser {
    state: ConditionalState,
    condition_container: TokenContainer,
}

impl ConditionalParser {
    pub fn new() -> ConditionalParser {
        ConditionalParser {
            state: ConditionalState::Nothing,
            condition_container: TokenContainer::new(),
        }
    }
}

impl<T> SubParser<T> for ConditionalParser
where
    T: Logger,
{
    fn check(&self, c: char) -> bool {
        match c {
            '?' => true,
            _ => false,
        }
    }

    fn identify(&self) -> &str {
        "Conditional Parser"
    }

    fn reset(&mut self) {
        self.state = ConditionalState::Nothing;
        self.condition_container.clear();
    }

    fn parse(
        &mut self,
        controller: &mut Controller<T>,
        parser_data: &mut ParserContainer,
        char_container: &mut CharContainer,
        token_container: &mut TokenContainer,
    ) -> Result<bool, Error> {
        while !parser_data.is_done() {
            let (c, ci, li) = parser_data.get_as_tuple();
            {
                controller.get_logger_mut().parser_next_char(c, ci, li);
            }
        }
        Err(Error::ImpossibleState)
    }
}
