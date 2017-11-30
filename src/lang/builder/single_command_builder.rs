use super::super::parser::token::Token;
use super::super::parser::token_container::TokenContainer;
use super::command_container::CommandContainer;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;
use super::sub_builder::SubBuilder;
use super::command::Command;

pub struct SingleCommandBuilder {}

impl SingleCommandBuilder {
    pub fn new() -> SingleCommandBuilder {
        SingleCommandBuilder {}
    }
}

impl<T> SubBuilder<T> for SingleCommandBuilder
where
    T: Logger,
{
    fn check(&self, token: &Token) -> bool {
        match *token {
            Token::Run | Token::Dereference => true,
            _ => false,
        }
    }

    fn presedence(&self) -> u64 {
        2
    }

    fn identify(&self) -> &str {
        "Run Builder"
    }

    fn do_clear(&self) -> bool {
        false
    }

    fn build(
        &mut self,
        controller: &mut Controller<T>,
        token_container: &mut TokenContainer,
        command_container: &mut CommandContainer,
        current_register: &mut usize,
    ) -> Result<(), Error> {
        // check the token to the right
        let right_counter;
        if let Some(reg_counter) = token_container.get_right_register_and_use() {
            right_counter = reg_counter;
        } else {
            return Err(Error::InvalidRightRegisterAccess);
        }
        if let Some(token) = token_container.get_slice_token_mut() {
            match *token {
                Token::Run => {
                    command_container.add_command(
                        controller,
                        Command::Run(
                            *current_register,
                            right_counter,
                        ),
                    );
                }
                Token::Dereference => {
                    command_container.add_command(
                        controller,
                        Command::Dereference(*current_register, right_counter),
                    );
                }
                _ => return Err(Error::TokenMismatch),
            };
            token.set_as_register(*current_register);
            *current_register += 1;
            return Ok(());
        }
        Err(Error::TokenMismatch)
    }
}
