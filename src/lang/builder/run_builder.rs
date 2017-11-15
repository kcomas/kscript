
use super::super::parser::token::Token;
use super::super::parser::token_container::TokenContainer;
use super::command_container::CommandContainer;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;
use super::sub_builder::SubBuilder;
use super::command::Command;

pub struct RunBuilder {}

impl RunBuilder {
    pub fn new() -> RunBuilder {
        RunBuilder {}
    }
}

impl<T> SubBuilder<T> for RunBuilder
where
    T: Logger,
{
    fn check(&self, token: &Token) -> bool {
        match *token {
            Token::Run => true,
            _ => false,
        }
    }

    fn presedence(&self) -> u64 {
        2
    }

    fn identify(&self) -> &str {
        "Run Builder"
    }

    fn build(
        &mut self,
        controller: &mut Controller<T>,
        token_container: &mut TokenContainer,
        command_container: &mut CommandContainer,
        current_register: &mut usize,
    ) -> Result<(), Error> {
        // check the token to the right
        if let Some(reg_counter) = token_container.get_right_register_and_use() {
            {
                command_container.add_command(
                    controller,
                    Command::Run(
                        *current_register,
                        reg_counter,
                    ),
                );
            }
            *current_register += 1;
        } else {
            return Err(Error::InvalidRegisterAccess);
        }
        if let Some(ref mut token) = token_container.get_slice_token_mut() {
            token.set_as_register(*current_register);
            return Ok(());
        }
        Err(Error::TokenMismatch)
    }
}
