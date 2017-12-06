
use super::super::parser::token::Token;
use super::super::parser::token_container::TokenContainer;
use super::command_container::CommandContainer;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;
use super::sub_builder::SubBuilder;
use super::command::Command;
use super::util::get_left_and_right;

pub struct DoubleCommandBuilder {}

impl DoubleCommandBuilder {
    pub fn new() -> DoubleCommandBuilder {
        DoubleCommandBuilder {}
    }
}

impl<T> SubBuilder<T> for DoubleCommandBuilder
where
    T: Logger,
{
    fn check(&self, token: &Token) -> bool {
        match *token {
            Token::Assign | Token::TakeReference | Token::Push => true,
            _ => false,
        }
    }

    fn presedence(&self) -> u64 {
        1
    }

    fn identify(&self) -> &str {
        "Assign Builder"
    }

    fn do_clear(&self) -> bool {
        true
    }

    fn build(
        &mut self,
        controller: &mut Controller<T>,
        token_container: &mut TokenContainer,
        command_container: &mut CommandContainer,
        _current_register: &mut usize,
    ) -> Result<(), Error> {

        let (left_counter, right_counter) = get_left_and_right(token_container)?;

        if let Some(token) = token_container.get_slice_token_mut() {
            match *token {
                Token::Assign => {
                    command_container.add_command(
                        controller,
                        Command::Assign(
                            left_counter,
                            right_counter,
                        ),
                    );
                }
                Token::TakeReference => {
                    command_container.add_command(
                        controller,
                        Command::TakeReference(left_counter, right_counter),
                    );
                }
                Token::Push => {
                    command_container.add_command(
                        controller,
                        Command::Push(left_counter, right_counter),
                    );
                }
                _ => return Err(Error::TokenMismatch),
            };
            *token = Token::Used;
            return Ok(());
        }
        Err(Error::TokenMismatch)
    }
}
