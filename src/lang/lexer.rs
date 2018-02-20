use std::iter::Peekable;
use std::str::Chars;
use super::error::LexerError;
use super::token::{Token, TokenBody};

fn peek_next_char(iter: &mut Peekable<Chars>) -> Result<char, LexerError> {
    match iter.peek() {
        Some(c) => Ok(*c),
        None => Err(LexerError::EndOfFile),
    }
}

#[derive(Debug)]
pub struct Matcher {
    base: char,
    base_token: Token,
    chars: Vec<Vec<char>>,
    tokens: Vec<Vec<Token>>,
}

impl Matcher {
    pub fn new(
        base: char,
        base_token: Token,
        chars: Vec<Vec<char>>,
        tokens: Vec<Vec<Token>>,
    ) -> Matcher {
        Matcher {
            base: base,
            base_token: base_token,
            chars: chars,
            tokens: tokens,
        }
    }

    pub fn is_match(&self, c: char) -> bool {
        self.base == c
    }

    pub fn do_match(&self, iter: &mut Peekable<Chars>) -> Result<Token, LexerError> {
        let mut token = self.base_token.clone();
        let mut outer_index = 0;

        while outer_index < self.chars.len() {
            let c = peek_next_char(iter)?;
            let mut inner_index = 0;
            while inner_index < self.chars[outer_index].len() {
                if c == self.chars[outer_index][inner_index] {
                    token = self.tokens[outer_index][inner_index].clone();
                    iter.next();
                    break;
                }
                inner_index += 1;
            }
            if inner_index == self.chars[outer_index].len() {
                break;
            }
            outer_index += 1;
        }
        Ok(token)
    }
}

pub fn create_matchers() -> Vec<Matcher> {
    vec![
        Matcher::new(
            '=',
            Token::Assign,
            vec![vec!['=', '<', '>']],
            vec![vec![Token::Equals, Token::EqualsGreater, Token::EqualsLess]],
        ),
        Matcher::new('+', Token::Add, Vec::new(), Vec::new()),
        Matcher::new('-', Token::Sub, Vec::new(), Vec::new()),
        Matcher::new(
            '!',
            Token::Not,
            vec![vec!['=']],
            vec![vec![Token::NotEquals]],
        ),
        Matcher::new('.', Token::CallSelf, Vec::new(), Vec::new()),
        Matcher::new(
            '>',
            Token::Greater,
            vec![vec!['>'], vec!['>']],
            vec![vec![Token::IoWrite], vec![Token::IoAppend]],
        ),
        Matcher::new('?', Token::If, Vec::new(), Vec::new()),
    ]
}

macro_rules! push_body {
    ($body:expr, $section:expr) => {
        if $section.len() > 0 {
            $body.push($section);
            $section = Vec::new();
        }
    };
}

pub fn lex(iter: &mut Peekable<Chars>, matchers: &Vec<Matcher>) -> Result<TokenBody, LexerError> {
    let mut token_body = Vec::new();
    let mut current_token_section = Vec::new();

    loop {
        let c = match iter.peek() {
            Some(c) => *c,
            None => break,
        };

        match c {
            '\n' | ';' => {
                iter.next();
                let next_c = match iter.peek() {
                    Some(c) => Some(*c),
                    None => None,
                };
                if let Some(c) = next_c {
                    if c == ';' {
                        current_token_section.push(Token::Return);
                    }
                }
                push_body!(token_body, current_token_section);
            }
            '#' => current_token_section.push(load_comment(iter)?),
            '{' => {
                iter.next();
                current_token_section.push(Token::Block(lex(iter, matchers)?));
            }
            '}' => {
                iter.next();
                push_body!(token_body, current_token_section);
                return Ok(token_body);
            }
            '(' => {
                iter.next();
                current_token_section.push(Token::Group(lex(iter, matchers)?));
            }
            ')' => {
                iter.next();
                push_body!(token_body, current_token_section);
                return Ok(token_body);
            }
            '_' | 'a'...'z' | 'A'...'Z' | '0'...'9' => {
                current_token_section.push(load_var_number(iter)?);
            }
            '"' => current_token_section.push(load_string(iter)?),
            _ => {
                if let Some(token) = run_matcher(c, iter, matchers)? {
                    current_token_section.push(token);
                } else {
                    iter.next();
                }
            }
        };
    }

    push_body!(token_body, current_token_section);
    Ok(token_body)
}

fn run_matcher(
    c: char,
    iter: &mut Peekable<Chars>,
    matchers: &Vec<Matcher>,
) -> Result<Option<Token>, LexerError> {
    for matcher in matchers.iter() {
        if matcher.is_match(c) {
            iter.next();
            return Ok(Some(matcher.do_match(iter)?));
        }
    }
    Ok(None)
}

fn load_comment(iter: &mut Peekable<Chars>) -> Result<Token, LexerError> {
    let mut string = String::new();

    loop {
        let c = peek_next_char(iter)?;
        if c == '\n' {
            break;
        }
        string.push(c);
        iter.next();
    }

    Ok(Token::Comment(string))
}

fn load_var_number(iter: &mut Peekable<Chars>) -> Result<Token, LexerError> {
    let mut is_integer = true;
    let mut is_float = false;

    let mut var_num = String::new();

    loop {
        let c = peek_next_char(iter)?;
        match c {
            '_' | 'a'...'z' | 'A'...'Z' => {
                is_integer = false;
                var_num.push(c);
                iter.next();
            }
            '0'...'9' => {
                var_num.push(c);
                iter.next();
            }
            '.' => {
                if !is_integer || is_float {
                    return Err(LexerError::InvalidFloat);
                }
                is_float = true;
                var_num.push(c);
                iter.next();
            }
            _ => break,
        };
    }

    let token = if is_float {
        Token::Float(var_num.parse().unwrap())
    } else if is_integer {
        Token::Integer(var_num.parse().unwrap())
    } else {
        Token::Var(var_num)
    };

    Ok(token)
}

fn load_string(iter: &mut Peekable<Chars>) -> Result<Token, LexerError> {
    iter.next();
    let mut string = String::new();

    loop {
        let c = peek_next_char(iter)?;

        match c {
            '"' => break,
            '\\' => {
                iter.next();
                let c = peek_next_char(iter)?;
                match c {
                    'n' => string.push('\n'),
                    't' => string.push('\t'),
                    '\\' => string.push('\\'),
                    _ => return Err(LexerError::InvalidStringEscape),
                };
            }
            _ => string.push(c),
        };
        iter.next();
    }

    iter.next();

    Ok(Token::String(string))
}
