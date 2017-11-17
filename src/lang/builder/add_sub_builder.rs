
use super::super::parser::token::Token;
use super::super::parser::token_container::TokenContainer;
use super::command_container::CommandContainer;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;
use super::sub_builder::SubBuilder;
use super::command::Command;
use super::util::get_left_and_right;

pub struct AddSubBuilder {}

impl AddSubBuilder {
    pub fn new() -> AddSubBuilder {
        AddSubBuilder {}
    }
}

impl<T> SubBuilder<T> for AddSubBuilder
where
    T: Logger,
{
    fn check(&self, token: &Token) -> bool {
        match *token {
            Token::Addition | Token::Subtract => true,
            _ => false,
        }
    }

    fn presedence(&self) -> u64 {
        1
    }

    fn identify(&self) -> &str {
        "Add Sub Builder"
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
        let (left_counter, right_counter) = get_left_and_right(token_container)?;

        if let Some(token) = token_container.get_slice_token_mut() {
            match *token {
                Token::Addition => {
                    *current_register += 1;
                    command_container.add_command(
                        controller,
                        Command::Addition(
                            *current_register,
                            left_counter,
                            right_counter,
                        ),
                    );
                }
                Token::Subtract => {
                    *current_register += 1;
                    command_container.add_command(
                        controller,
                        Command::Subtract(
                            *current_register,
                            left_counter,
                            right_counter,
                        ),
                    );
                }
                _ => return Err(Error::TokenMismatch),
            }
            token.set_as_register(*current_register);
            return Ok(());
        }
        Err(Error::TokenMismatch)
    }
}
