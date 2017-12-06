use super::super::parser::token::Token;
use super::super::parser::token_container::TokenContainer;
use super::command_container::CommandContainer;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;
use super::sub_builder::SubBuilder;
use super::command::Command;

pub struct ModifierBuilder {}

impl ModifierBuilder {
    pub fn new() -> ModifierBuilder {
        ModifierBuilder {}
    }
}

impl<T> SubBuilder<T> for ModifierBuilder
where
    T: Logger,
{
    fn check(&self, token: &Token) -> bool {
        match *token {
            Token::Dereference | Token::Cast(_) => true,
            _ => false,
        }
    }

    fn presedence(&self) -> u64 {
        3
    }

    fn identify(&self) -> &str {
        "Modifier Builder"
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
        } else if let Some(reg_counter) = token_container.get_left_register_and_use() {
            right_counter = reg_counter;
        } else {
            return Err(Error::InvalidRightRegisterAccess);
        }
        if let Some(token) = token_container.get_slice_token_mut() {
            match *token {
                Token::Dereference => {
                    command_container.add_command(
                        controller,
                        Command::Dereference(*current_register, right_counter),
                    );
                }
                Token::Cast(ref cast_to) => {
                    command_container.add_command(
                        controller,
                        Command::Cast(
                            cast_to.clone(),
                            *current_register,
                            right_counter,
                        ),
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
