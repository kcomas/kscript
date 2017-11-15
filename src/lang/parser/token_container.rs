
use super::super::controller::Controller;
use super::super::logger::Logger;
use super::token::Token;

pub struct CurrentSlice {
    start: usize,
    end: usize,
    position: usize,
}

pub struct TokenContainer {
    tokens: Vec<Token>,
    current_token: usize,
    current_slice: CurrentSlice,
}

impl TokenContainer {
    pub fn new() -> TokenContainer {
        TokenContainer {
            tokens: Vec::new(),
            current_token: 0,
            current_slice: CurrentSlice {
                start: 0,
                end: 0,
                position: 0,
            },
        }
    }

    pub fn is_done(&self) -> bool {
        self.current_token >= self.tokens.len()
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

    pub fn get_mut(&mut self, i: usize) -> Option<&mut Token> {
        self.tokens.get_mut(i)
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn get_current_token(&self) -> &Token {
        match self.get(self.current_token) {
            Some(ref token) => token,
            None => &Token::Used,
        }
    }

    pub fn is_current_token_end(&self) -> bool {
        self.get_current_token().is_end()
    }

    pub fn set_current_end_as_used(&mut self) {
        if self.is_current_token_end() {
            self.tokens[self.current_token] = Token::Used;
        }
    }

    pub fn inc_token(&mut self) {
        self.current_token += 1;
    }

    pub fn update_slice_end(&mut self) {
        self.current_slice.end = self.current_token;
    }

    pub fn update_slice_start(&mut self) {
        self.current_slice.start = self.current_token;
        self.current_slice.position = self.current_token;
    }

    pub fn reset_slice_position(&mut self) {
        self.current_slice.position = self.current_slice.start;
    }

    pub fn in_slice(&self) -> bool {
        self.current_slice.position <= self.current_slice.end
    }

    pub fn get_slice_token(&self) -> Option<&Token> {
        let pos = self.current_slice.position;
        self.get(pos)
    }

    pub fn get_slice_token_mut(&mut self) -> Option<&mut Token> {
        let pos = self.current_slice.position;
        self.get_mut(pos)
    }

    pub fn inc_slice_position(&mut self) {
        self.current_slice.position += 1;
    }

    pub fn get_register_and_use(&mut self, mut pos: usize, inc_up: bool) -> Option<usize> {
        let start = self.current_slice.start;
        let end = self.current_slice.end;
        loop {
            if let Some(token) = self.get_mut(pos) {
                if let Some(reg_counter) = token.is_register() {
                    *token = Token::Used;
                    return Some(reg_counter);
                } else {
                    match *token {
                        Token::Used => {
                            if inc_up && pos < end {
                                pos += 1;
                            } else if pos > start {
                                pos -= 1;
                            }
                        }
                        _ => break,
                    };
                }
            }
        }
        None
    }

    pub fn get_right_register_and_use(&mut self) -> Option<usize> {
        // get the right value if register and set to used
        let pos = self.current_slice.position + 1;
        self.get_register_and_use(pos, true)
    }

    pub fn get_left_register_and_use(&mut self) -> Option<usize> {
        let pos = self.current_slice.position - 1;
        self.get_register_and_use(pos, false)
    }

    pub fn get_slice_token_index(&self) -> usize {
        self.current_slice.position
    }

    pub fn set_slice_token_index(&mut self, idx: usize) {
        self.current_slice.position = idx;
    }
}
