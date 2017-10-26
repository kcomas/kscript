
use super::sub_parser::SubParser;
use super::token::Token;
use super::super::logger::Logger;
use super::super::controller::Controller;

#[derive(Debug)]
pub struct EndParser {}

impl SubParser for EndParser {
    fn check(c: char) -> bool {
        match c {
            ';' | '\n' => true,
            _ => false,
        }
    }

    fn parse<T: Logger>(
        controller: &mut Controller<T>,
        text_vec: &Vec<char>,
        current_char: &mut usize,
        current_line: &mut usize,
        current_chars: &mut Vec<char>,
        tokens: &mut Vec<Token>,
    ) -> Result<(), String> {
        match text_vec[*current_char] {
            ';' => {
                tokens.push(Token::End);
                *current_char += 1;
            }
            '\n' => {
                tokens.push(Token::End);
                *current_line += 1;
                *current_char += 1;
            }
            _ => return Err("derp".to_string()),
        };
        Ok(())
    }
}
