
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

pub enum ConditionalParserState {
    Nothing,
    MabeBlocks,
    TrueStatements,
    MabeFalse,
    FalseStatements,
}

pub struct ConditionalParser {
    state: ConditionalParserState,
}

impl ConditionalParser {
    pub fn new() -> ConditionalParser {
        ConditionalParser { state: ConditionalParserState::Nothing }
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
        let mut cond = None;
        let mut true_tokens: Vec<Token> = Vec::new();
        let mut false_tokens: Vec<Token> = Vec::new();
        let mut done = false;

        {
            let mut true_statements = TokenContainer::new(&mut true_tokens);
            let mut false_statements = TokenContainer::new(&mut false_tokens);
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
                                let mut cb = CondBuilder::new();
                                cond = Some(cb.parse(controller, parser_data, char_container)?);
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
                            '{' => {
                                parser_data.inc_char();
                                ConditionalParserState::TrueStatements
                            }
                            _ => {
                                if let Some(token) = cond {
                                    token_container.add_token(controller, token);
                                    return Ok(false);
                                }
                                return Err(Error::InvalidConditional(c, ci, li));
                            }
                        }
                    }
                    ConditionalParserState::TrueStatements => {
                        let mut parsers = top_level_parsers();

                        do_parse(
                            parser_data,
                            controller,
                            char_container,
                            &mut true_statements,
                            &mut parsers,
                        )?;

                        ConditionalParserState::MabeFalse
                    }
                    ConditionalParserState::MabeFalse => {
                        match c {
                            ' ' | '\n' => {
                                parser_data.inc_char();
                                ConditionalParserState::MabeFalse
                            }
                            '{' => {
                                parser_data.inc_char();
                                ConditionalParserState::FalseStatements
                            }
                            _ => {
                                if true_statements.len() == 0 {
                                    return Err(Error::InvalidIfBlock(c, ci, li));
                                }
                                done = true;
                                break;
                            }
                        }
                    }
                    ConditionalParserState::FalseStatements => {
                        let mut parsers = top_level_parsers();

                        do_parse(
                            parser_data,
                            controller,
                            char_container,
                            &mut false_statements,
                            &mut parsers,
                        )?;

                        if false_statements.len() == 0 {
                            return Err(Error::InvalidIfBlock(c, ci, li));
                        }
                        done = true;
                        break;
                    }
                };
            }
        }
        if done {
            token_container.add_token(
                controller,
                Token::If(Box::new(cond.unwrap()), true_tokens, false_tokens),
            );

            return Ok(false);
        }
        Err(Error::ImpossibleState)
    }
}
