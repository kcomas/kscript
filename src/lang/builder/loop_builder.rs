
use super::super::parser::token::Token;
use super::super::parser::token_container::TokenContainer;
use super::command_container::CommandContainer;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;
use super::sub_builder::SubBuilder;
use super::command::Command;
use super::util::{token_to_data_type, create_new_command_container, top_level_builders};

pub struct LoopBuilder {}

impl LoopBuilder {
    pub fn new() -> LoopBuilder {
        LoopBuilder {}
    }
}

impl<T> SubBuilder<T> for LoopBuilder
where
    T: Logger,
{
    fn check(&self, token: &Token) -> bool {
        match *token {
            Token::Loop(_, _) => true,
            _ => false,
        }
    }

    fn presedence(&self) -> u64 {
        2
    }

    fn identify(&self) -> &str {
        "Loop Builder"
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
        if let Some(token) = token_container.get_slice_token_mut() {
            match *token {
                Token::Loop(ref mut conditional, ref statements) => {
                    let mabe_cond = token_to_data_type(
                        controller,
                        command_container,
                        current_register,
                        conditional,
                    )?;

                    if let Some(is_cond) = mabe_cond {
                    } else {
                        return Err(Error::InvalidConditonalToken);
                    }
                }
                _ => return Err(Error::TokenMismatch),
            }
        }
        Err(Error::TokenMismatch)
    }
}
