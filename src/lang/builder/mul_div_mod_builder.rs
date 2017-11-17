
use super::super::parser::token::Token;
use super::super::parser::token_container::TokenContainer;
use super::command_container::CommandContainer;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;
use super::sub_builder::SubBuilder;
use super::command::Command;
use super::util::get_left_and_right;

pub struct MulDivModBuilder {}

impl MulDivModBuilder {
    pub fn new() -> MulDivModBuilder {
        MulDivModBuilder {}
    }
}

impl<T> SubBuilder<T> for MulDivModBuilder
where
    T: Logger,
{
    fn check(&self, token: &Token) -> bool {
        match *token {
            Token::Multiply | Token::Divide | Token::Modulus => true,
            _ => false,
        }
    }

    fn presedence(&self) -> u64 {
        2
    }

    fn identify(&self) -> &str {
        "Mul Div Mod Builder"
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
            *current_register += 1;
            match *token {
                Token::Multiply => {
                    command_container.add_command(
                        controller,
                        Command::Multiply(
                            *current_register,
                            left_counter,
                            right_counter,
                        ),
                    );
                }
                Token::Divide => {
                    command_container.add_command(
                        controller,
                        Command::Divide(
                            *current_register,
                            left_counter,
                            right_counter,
                        ),
                    );
                }
                Token::Modulus => {
                    command_container.add_command(
                        controller,
                        Command::Modulus(
                            *current_register,
                            left_counter,
                            right_counter,
                        ),
                    );
                }
                _ => return Err(Error::TokenMismatch),
            };
            token.set_as_register(*current_register);
            return Ok(());
        }
        Err(Error::TokenMismatch)
    }
}
