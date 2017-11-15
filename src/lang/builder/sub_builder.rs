
use super::super::parser::token::Token;
use super::super::parser::token_container::TokenContainer;
use super::command_container::CommandContainer;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;

pub trait SubBuilder<T: Logger> {
    fn check(&self, token: &Token) -> bool;

    fn presedence(&self) -> u64;

    fn identify(&self) -> &str;

    fn reset(&mut self) {}

    fn do_clear(&self) -> bool;

    fn build(
        &mut self,
        controller: &mut Controller<T>,
        token_container: &mut TokenContainer,
        command_container: &mut CommandContainer,
        current_register: &mut usize,
    ) -> Result<(), Error>;
}
