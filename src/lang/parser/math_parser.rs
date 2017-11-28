
use super::token::Token;
use super::token_container::TokenContainer;
use super::parser_container::ParserContainer;
use super::char_container::CharContainer;
use super::sub_parser::SubParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;
use super::util::{do_parse, math_parsers};

pub struct MathParser {}

impl MathParser {
    pub fn new() -> MathParser {
        MathParser {}
    }
}

impl<T> SubParser<T> for MathParser
where
    T: Logger,
{
    fn check(&self, c: char) -> bool {
        match c {
            '(' | ')' => true,
            _ => false,
        }
    }

    fn identify(&self) -> &str {
        "Math Parser"
    }

    fn reset(&mut self) {}

    fn parse(
        &mut self,
        controller: &mut Controller<T>,
        parser_data: &mut ParserContainer,
        char_container: &mut CharContainer,
        token_container: &mut TokenContainer,
    ) -> Result<bool, Error> {
        match parser_data.get_current_char() {
            '(' => {
                let mut tokens: Vec<Token> = Vec::new();
                {
                    let mut math_container = TokenContainer::new(&mut tokens);
                    parser_data.inc_char();
                    let mut parsers = math_parsers();

                    do_parse(
                        parser_data,
                        controller,
                        char_container,
                        &mut math_container,
                        &mut parsers,
                    )?;
                }

                token_container.add_token(controller, Token::Math(tokens));
            }
            ')' => {
                parser_data.inc_char();
                return Ok(true);
            }
            _ => {
                let (c, ci, li) = parser_data.get_as_tuple();
                return Err(Error::CheckMismatch(c, ci, li));
            }
        }
        Ok(false)
    }
}
