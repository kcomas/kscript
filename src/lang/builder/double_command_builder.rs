
use super::super::parser::token::Token;
use super::super::parser::token_container::TokenContainer;
use super::command_container::CommandContainer;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;
use super::sub_builder::SubBuilder;
use super::command::Command;

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
            Token::Assign => true,
            _ => false,
        }
    }

    fn presedence(&self) -> u64 {
        1
    }

    fn identify(&self) -> &str {
        "Assign Builder"
    }

    fn build(
        &mut self,
        controller: &mut Controller<T>,
        token_container: &mut TokenContainer,
        command_container: &mut CommandContainer,
        _current_register: &mut usize,
    ) -> Result<(), Error> {
        let left_counter;
        let right_counter;

        if let Some(reg_counter) = token_container.get_right_register_and_use() {
            right_counter = reg_counter;
        } else {
            return Err(Error::InvalidRightRegisterAccess);
        }

        if let Some(reg_counter) = token_container.get_left_register_and_use() {
            left_counter = reg_counter;
        } else {
            return Err(Error::InvalidLeftRegisterAccess);
        }

        if let Some(token) = token_container.get_slice_token_mut() {
            match *token {
                Token::Assign => {
                    *token = Token::Used;
                    command_container.add_command(
                        controller,
                        Command::Assign(
                            left_counter,
                            right_counter,
                        ),
                    );
                }
                _ => return Err(Error::TokenMismatch),
            }
            return Ok(());
        }

        Err(Error::TokenMismatch)
    }
}
