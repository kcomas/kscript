
use super::token::Token;
use super::token_container::TokenContainer;
use super::char_container::CharContainer;
use super::parser_container::ParserContainer;
use super::sub_parser::SubParser;
use super::var_parser::VarParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;
use super::util::do_parse_single;

pub enum RefParserState {
    Nothing,
    Load,
}

pub struct RefParser {
    state: RefParserState,
}

impl RefParser {
    pub fn new() -> RefParser {
        RefParser { state: RefParserState::Nothing }
    }
}

impl<T> SubParser<T> for RefParser
where
    T: Logger,
{
    fn check(&self, c: char) -> bool {
        match c {
            '&' => true,
            _ => false,
        }
    }

    fn identify(&self) -> &str {
        "Ref Parser"
    }

    fn reset(&mut self) {
        self.state = RefParserState::Nothing;
    }

    fn parse(
        &mut self,
        controller: &mut Controller<T>,
        parser_data: &mut ParserContainer,
        char_container: &mut CharContainer,
        token_container: &mut TokenContainer,
    ) -> Result<bool, Error> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut container = TokenContainer::new(&mut tokens);
        let mut var_parser: [Box<SubParser<T>>; 1] = [Box::new(VarParser::new())];

        while !parser_data.is_done() {
            let (c, ci, li) = parser_data.get_as_tuple();
            {
                controller.get_logger_mut().parser_next_char(c, ci, li);
            }
            self.state = match self.state {
                RefParserState::Nothing => {
                    match c {
                        '&' => {
                            parser_data.inc_char();
                            RefParserState::Load
                        }
                        _ => return Err(Error::CheckMismatch(c, ci, li)),
                    }
                }
                RefParserState::Load => {
                    let (_exit, used) = do_parse_single(
                        c,
                        parser_data,
                        controller,
                        char_container,
                        &mut container,
                        &mut var_parser,
                    )?;

                    match used {
                        true => {
                            if container.len() != 1 {
                                return Err(Error::InvalidRef(c, ci, li));
                            }

                            let token = container.pop().unwrap();
                            token_container.add_token(controller, Token::Ref(Box::new(token)));
                            return Ok(false);
                        }
                        false => {
                            parser_data.inc_char();
                            RefParserState::Load
                        }
                    }
                }
            };
        }
        Err(Error::ImpossibleState)
    }
}
