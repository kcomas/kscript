
use super::token::Token;
use super::token_container::TokenContainer;
use super::char_container::CharContainer;
use super::parser_container::ParserContainer;
use super::sub_parser::SubParser;
use super::cond_builder::CondBuilder;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;

pub enum ConditionalParserState {
    Nothing,
    MabeBlocks,
    // TrueStatements,
    // FalseStatements,
}

pub struct ConditionalParser {
    state: ConditionalParserState,
    cond: Option<Token>,
}

impl ConditionalParser {
    pub fn new() -> ConditionalParser {
        ConditionalParser {
            state: ConditionalParserState::Nothing,
            cond: None,
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
            self.state = match self.state {
                ConditionalParserState::Nothing => {
                    match c {
                        '?' => {
                            parser_data.inc_char();
                            let mut cond = CondBuilder::new();
                            self.cond = Some(cond.parse(controller, parser_data, char_container)?);
                            ConditionalParserState::MabeBlocks
                        }
                        _ => return Err(Error::CheckMismatch(c, ci, li)),
                    }
                }
                ConditionalParserState::MabeBlocks => {
                    match c {
                        ' ' | '\n' => {
                            parser_data.inc_char();
                            ConditionalParserState::MabeBlocks
                        }
                        _ => {
                            if let Some(ref token) = self.cond {
                                token_container.add_token(controller, token.clone());
                                return Ok(false);
                            }
                            return Err(Error::InvalidConditional(c, ci, li));
                        }
                    }
                }
            };
        }
        Err(Error::ImpossibleState)
    }
}
