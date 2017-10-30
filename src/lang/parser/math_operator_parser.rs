
use super::token::Token;
use super::token_container::TokenContainer;
use super::parser_container::ParserContainer;
use super::char_container::CharContainer;
use super::sub_parser::SubParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;

pub struct MathOperatorParser {}

impl MathOperatorParser {
    pub fn new() -> MathOperatorParser {
        MathOperatorParser {}
    }
}

impl<T> SubParser<T> for MathOperatorParser
where
    T: Logger,
{
    fn check(&self, c: char) -> bool {
        match c {
            '+' | '-' | '*' | '/' | '%' | '^' => true,
            _ => false,
        }
    }

    fn identify(&self) -> &str {
        "Math Operator Parser"
    }

    fn parse(
        &mut self,
        controller: &mut Controller<T>,
        parser_data: &mut ParserContainer,
        _char_container: &mut CharContainer,
        token_container: &mut TokenContainer,
    ) -> Result<bool, Error> {
        match parser_data.get_current_char() {
            '+' => {
                token_container.add_token(controller, Token::Add);
                parser_data.inc_char();
            }
            '-' => {
                token_container.add_token(controller, Token::Subtract);
                parser_data.inc_char();
            }
            '*' => {
                token_container.add_token(controller, Token::Multiply);
                parser_data.inc_char();
            }
            '/' => {
                token_container.add_token(controller, Token::Divide);
                parser_data.inc_char();
            }
            '%' => {
                token_container.add_token(controller, Token::Modulus);
                parser_data.inc_char();
            }
            '^' => {
                token_container.add_token(controller, Token::Exponent);
                parser_data.inc_char();
            }
            _ => {
                let (c, ci, li) = parser_data.get_as_tuple();
                return Err(Error::CheckMismatch(c, ci, li));
            }
        }
        Ok(false)
    }
}
