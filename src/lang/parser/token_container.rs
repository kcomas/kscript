
use super::super::controller::Controller;
use super::super::logger::Logger;
use super::token::Token;

#[derive(Debug)]
pub struct TokenContainer {
    tokens: Vec<Token>,
}

impl TokenContainer {
    pub fn new() -> TokenContainer {
        TokenContainer { tokens: Vec::new() }
    }

    pub fn add_token<T: Logger>(&mut self, controller: &mut Controller<T>, token: Token) {
        {
            controller.get_logger_mut().parser_add_token(token.clone());
        }
        self.tokens.push(token);
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}
