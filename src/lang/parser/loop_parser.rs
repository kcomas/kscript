
use super::token::Token;
use super::token_container::TokenContainer;
use super::char_container::CharContainer;
use super::parser_container::ParserContainer;
use super::sub_parser::SubParser;
use super::cond_builder::CondBuilder;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;
use super::util::{do_parse, top_level_parsers};

pub enum LoopParserState {
    Nothing,
    MabeStatements,
    Statements,
}

pub struct LoopParser {
    state: LoopParserState,
    cond: Option<Token>,
    statements: TokenContainer,
}

impl LoopParser {
    pub fn new() -> LoopParser {
        LoopParser {
            state: LoopParserState::Nothing,
            cond: None,
            statements: TokenContainer::new(),
        }
    }
}

impl<T> SubParser<T> for LoopParser
where
    T: Logger,
{
    fn check(&self, c: char) -> bool {
        match c {
            '$' => true,
            _ => false,
        }
    }

    fn identify(&self) -> &str {
        "Loop Parser"
    }

    fn reset(&mut self) {
        self.state = LoopParserState::Nothing;
        self.cond = None;
        self.statements.clear();
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
                LoopParserState::Nothing => {
                    match c {
                        '$' => {
                            parser_data.inc_char();
                            let mut cond = CondBuilder::new();
                            self.cond = Some(cond.parse(controller, parser_data, char_container)?);
                            LoopParserState::MabeStatements
                        }
                        _ => return Err(Error::CheckMismatch(c, ci, li)),
                    }
                }
                LoopParserState::MabeStatements => {
                    match c {
                        ' ' | '\n' => {
                            parser_data.inc_char();
                            LoopParserState::MabeStatements
                        }
                        '{' => {
                            parser_data.inc_char();
                            LoopParserState::Statements
                        }
                        _ => return Err(Error::InvalidLoop(c, ci, li)),
                    }
                }
                LoopParserState::Statements => {
                    let mut parsers = top_level_parsers();

                    do_parse(
                        parser_data,
                        controller,
                        char_container,
                        &mut self.statements,
                        &mut parsers,
                    )?;

                    if self.statements.len() == 0 {
                        return Err(Error::InvalidIfBlock(c, ci, li));
                    }

                    token_container.add_token(
                        controller,
                        Token::Loop(
                            Box::new(self.cond.clone().unwrap()),
                            self.statements.get_tokens().clone(),
                        ),
                    );
                    return Ok(false);
                }
            };
        }
        Err(Error::ImpossibleState)
    }
}
