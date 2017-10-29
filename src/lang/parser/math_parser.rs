
use super::token::Token;
use super::token_container::TokenContainer;
use super::parser_container::ParserContainer;
use super::char_container::CharContainer;
use super::sub_parser::SubParser;
use super::line_end_parser::LineEndParser;
use super::var_parser::VarParser;
use super::number_parser::NumberParser;
use super::math_operator_parser::MathOperatorParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;
use super::util::do_parse;

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

    fn parse(
        &mut self,
        controller: &mut Controller<T>,
        parser_data: &mut ParserContainer,
        char_container: &mut CharContainer,
        token_container: &mut TokenContainer,
    ) -> Result<(), Error> {
        let mut math_container = TokenContainer::new();
        match parser_data.get_current_char() {
            '(' => {
                parser_data.inc_char();
                let mut parsers: [Box<SubParser<T>>; 5] = [
                    Box::new(LineEndParser::new()),
                    Box::new(VarParser::new()),
                    Box::new(MathOperatorParser::new()),
                    Box::new(NumberParser::new()),
                    Box::new(MathParser::new()),
                ];

                if let Err(kerror) = do_parse(
                    parser_data,
                    controller,
                    5,
                    &mut parsers,
                    char_container,
                    &mut math_container,
                )
                {
                    return Err(kerror);
                }

                token_container.add_token(
                    controller,
                    Token::Math(
                        math_container.get_tokens().clone(),
                    ),
                );
            }
            ')' => {
                parser_data.inc_char();
                // flush the current token container
                let tc = token_container.get_tokens().clone();
                token_container.clear();
                token_container.add_token(controller, Token::Math(tc));
            }
            _ => {
                let (c, ci, li) = parser_data.get_as_tuple();
                return Err(Error::CheckMismatch(c, ci, li));
            }
        }
        Ok(())
    }
}
