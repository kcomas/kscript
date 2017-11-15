
use super::super::parser::token::Token;
use super::super::parser::token_container::TokenContainer;
use super::command_container::CommandContainer;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;
use super::sub_builder::SubBuilder;
use super::command::Command;

pub struct AssignBuilder {}

impl AssignBuilder {
    pub fn new() -> AssignBuilder {
        AssignBuilder {}
    }
}

impl<T> SubBuilder<T> for AssignBuilder
where
    T: Logger,
{
    fn check(&self, token: &Token) -> bool {
        match *token {
            Token::Assign => true,
            _ => false,
        }
    }

    fn presedence(&self) -> u64 {
        1
    }
}
