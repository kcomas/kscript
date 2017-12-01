
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
}

impl CondBuilder {
    pub fn new() -> CondBuilder {
        CondBuilder { state: CondBuilderState::ItemA }
    }

    pub fn parse<T: Logger>(
        &mut self,
        controller: &mut Controller<T>,
        parser_data: &mut ParserContainer,
        char_container: &mut CharContainer,
    ) -> Result<Token, Error> {
        let mut cond_tokens: Vec<Token> = Vec::new();
        let mut cond_container = TokenContainer::new(&mut cond_tokens);
        let mut parsers = conditional_parsers();
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
                        char_container,
                        &mut cond_container,
                        &mut parsers,
                    )?;

                    match used {
                        true => {
                            match parser_data.get_current_char() {
                                '[' | '|' => CondBuilderState::ItemA,
                                _ => CondBuilderState::Conditional,
                            }
                        }
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
                        char_container,
                        &mut cond_container,
                        &mut operator_parsers,
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
                        char_container,
                        &mut cond_container,
                        &mut parsers,
                    )?;

                    match used {
                        true => {
                            match parser_data.get_current_char() {
                                '[' | '|' => CondBuilderState::ItemB,
                                _ => {
                                    let item_b = cond_container.pop().unwrap();
                                    let cond = cond_container.pop().unwrap();
                                    let item_a = cond_container.pop().unwrap();
                                    let token = Token::Conditional(
                                        Box::new(item_a),
                                        Box::new(cond),
                                        Box::new(item_b),
                                    );
                                    {
                                        controller.get_logger_mut().parser_add_token(&token);
                                    }
                                    return Ok(token);
                                }
                            }
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
