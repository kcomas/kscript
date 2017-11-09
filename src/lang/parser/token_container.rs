
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
            controller.get_logger_mut().parser_add_token(&token);
        }
        self.tokens.push(token);
    }

    pub fn clear(&mut self) {
        self.tokens.clear();
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    pub fn get_tokens_mut(&mut self) -> &mut Vec<Token> {
        &mut self.tokens
    }

    pub fn merge_tokens(&mut self, new_tokens: &mut Vec<Token>) {
        self.tokens.append(new_tokens);
    }

    pub fn get(&self, i: usize) -> Option<&Token> {
        self.tokens.get(i)
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }
}
