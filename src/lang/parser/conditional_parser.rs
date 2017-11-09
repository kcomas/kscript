
use super::token::Token;
use super::token_container::TokenContainer;
use super::char_container::CharContainer;
use super::parser_container::ParserContainer;
use super::sub_parser::SubParser;
use super::operator_parser::OperatorParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;
use super::util::{do_parse_single, loop_conditional_parsers};

pub enum ConditionalParserState {
    Nothing,
    ItemA,
    Conditional,
    // ItemB,
    // MabeBlocks,
    // TrueStatements,
    // FalseStatements,
}

pub struct ConditionalParser {
    state: ConditionalParserState,
    condition_container: TokenContainer,
}

impl ConditionalParser {
    pub fn new() -> ConditionalParser {
        ConditionalParser {
            state: ConditionalParserState::Nothing,
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
        self.state = ConditionalParserState::Nothing;
        self.condition_container.clear();
    }

    fn parse(
        &mut self,
        controller: &mut Controller<T>,
        parser_data: &mut ParserContainer,
        char_container: &mut CharContainer,
        token_container: &mut TokenContainer,
    ) -> Result<bool, Error> {
        let (mut parsers, num_parsers) = loop_conditional_parsers();
        let operator_parsers = [Box::new(OperatorParser::new()); 1];

        while !parser_data.is_done() {
            let (c, ci, li) = parser_data.get_as_tuple();
            {
                controller.get_logger_mut().parser_next_char(c, ci, li);
            }
            self.state = match self.state {
                ConditionalParserState::Nothing => {
                    match c {
                        '?' => {
                            parser_data.inc_char();
                            ConditionalParserState::ItemA
                        }
                        _ => return Err(Error::CheckMismatch(c, ci, li)),
                    }
                }
                ConditionalParserState::ItemA => {
                    let (_exit, used) = do_parse_single(
                        c,
                        parser_data,
                        controller,
                        num_parsers,
                        &mut parsers,
                        char_container,
                        &mut self.condition_container,
                    )?;

                    match used {
                        true => ConditionalParserState::Conditional,
                        false => {
                            parser_data.inc_char();
                            ConditionalParserState::ItemA
                        }
                    }
                }
                ConditionalParserState::Conditional => return Ok(false),
            };
        }
        Err(Error::ImpossibleState)
    }
}
