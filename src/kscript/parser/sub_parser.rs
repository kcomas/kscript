
use super::token::Token;
use super::super::logger::Logger;
use super::super::controller::Controller;

pub trait SubParser {
    // if the current chars can be taken into this parser
    fn check(c: char) -> bool;

    fn parse<T: Logger>(
        controller: &mut Controller<T>,
        text_vec: &Vec<char>,
        current_char: &mut usize,
        current_line: &mut usize,
        current_chars: &mut Vec<char>,
        tokens: &mut Vec<Token>,
    ) -> Result<(), String>;
}
