
use super::token::Token;
use super::token_container::TokenContainer;
use super::char_container::CharContainer;
use super::parser_container::ParserContainer;
use super::sub_parser::SubParser;
use super::operator_parser::OperatorParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;
use super::util::{do_parse_single, conditional_parsers};

pub enum CondBuilderState {
    ItemA,
    Conditional,
    ItemB,
}

pub struct CondBuilder {
    state: CondBuilderState,
    cond_container: TokenContainer,
}

impl CondBuilder {
    pub fn new() -> CondBuilder {
        CondBuilder {
            state: CondBuilderState::ItemA,
            cond_container: TokenContainer::new(),
        }
    }

    pub fn parse<T: Logger>(
        &mut self,
        controller: &mut Controller<T>,
        parser_data: &mut ParserContainer,
        char_container: &mut CharContainer,
    ) -> Result<Token, Error> {
        let (mut parsers, num_parsers) = conditional_parsers();
        let mut operator_parsers: [Box<SubParser<T>>; 1] = [Box::new(OperatorParser::new())];

        while !parser_data.is_done() {
            let (c, ci, li) = parser_data.get_as_tuple();
            {
                controller.get_logger_mut().parser_next_char(c, ci, li);
            }
            self.state = match self.state {
                CondBuilderState::ItemA => {
                    let (_exit, used) = do_parse_single(
                        c,
                        parser_data,
                        controller,
                        num_parsers,
                        &mut parsers,
                        char_container,
                        &mut self.cond_container,
                    )?;

                    match used {
                        true => CondBuilderState::Conditional,
                        false => {
                            parser_data.inc_char();
                            CondBuilderState::ItemA
                        }
                    }
                }
                CondBuilderState::Conditional => {
                    let (_exit, used) = do_parse_single(
                        c,
                        parser_data,
                        controller,
                        1,
                        &mut operator_parsers,
                        char_container,
                        &mut self.cond_container,
                    )?;

                    match used {
                        true => CondBuilderState::ItemB,
                        false => {
                            parser_data.inc_char();
                            CondBuilderState::Conditional
                        }
                    }
                }
                CondBuilderState::ItemB => {
                    let (_exit, used) = do_parse_single(
                        c,
                        parser_data,
                        controller,
                        num_parsers,
                        &mut parsers,
                        char_container,
                        &mut self.cond_container,
                    )?;

                    match used {
                        true => {
                            let token = Token::Conditional(
                                Box::new(self.cond_container.get(0).unwrap().clone()),
                                Box::new(self.cond_container.get(1).unwrap().clone()),
                                Box::new(self.cond_container.get(2).unwrap().clone()),
                            );
                            self.cond_container.clear();
                            return Ok(token);
                        }
                        false => {
                            parser_data.inc_char();
                            CondBuilderState::ItemB
                        }
                    }
                }
            };
        }
        Err(Error::ImpossibleState)
    }
}
