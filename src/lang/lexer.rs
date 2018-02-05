use std::iter::Peekable;
use std::str::Chars;
use super::error::ParserError;
use super::token::{Token, TokenBody};

pub fn string_to_token(iter: &mut Peekable<Chars>) -> Result<TokenBody, ParserError> {
    let mut token_body = Vec::new();
    let mut current_token = Vec::new();

    loop {
        let c = match iter.peek() {
            Some(c) => *c,
            None => break,
        };
        match_token(c, iter, &mut token_body, &mut current_token)?;
    }
    update_token_body(&mut token_body, &mut current_token);
    Ok(token_body)
}

fn match_token(
    c: char,
    iter: &mut Peekable<Chars>,
    token_body: &mut TokenBody,
    current_token: &mut Vec<Token>,
) -> Result<(), ParserError> {
    match c {
        '\n' | ',' => {
            iter.next();
            update_token_body(token_body, current_token);
        }
        ';' => {
            iter.next();
            let c2 = match iter.peek() {
                Some(c) => *c,
                None => return Err(ParserError::InvalidReturn),
            };
            if c2 == ';' {
                iter.next();
                current_token.push(Token::Return);
            }
            update_token_body(token_body, current_token);
        }
        '0'...'9' | '_' | 'a'...'z' | 'A'...'Z' => current_token.push(load_var_number(iter)?),
        '#' => {
            current_token.push(load_comment(iter)?);
            update_token_body(token_body, current_token);
        }
        '?' | '+' | '-' | '.' => {
            let token = match c {
                '?' => Token::If,
                '+' => Token::Add,
                '-' => Token::Sub,
                '.' => Token::Call,
                _ => panic!("Impossible Symbol Mismatch"),
            };
            iter.next();
            current_token.push(token);
        }
        '=' => current_token.push(load_equals(iter)?),
        '>' => current_token.push(multi_char(
            iter,
            '>',
            vec![Token::Greater, Token::IoWrite, Token::IoAppend],
            ParserError::InvalidIoWriteIoAppendGreater,
        )?),
        '(' => current_token.push(Token::Group(combine_token(iter, ')', true)?)),
        '{' => {
            current_token.push(Token::Block(combine_token(iter, '}', false)?));
        }
        _ => {
            iter.next();
        }
    };
    Ok(())
}

fn update_token_body(token_body: &mut TokenBody, current_token: &mut Vec<Token>) {
    token_body.push(current_token.clone());
    current_token.clear();
}

fn peek_next_char(iter: &mut Peekable<Chars>, error: &ParserError) -> Result<char, ParserError> {
    if let Some(c) = iter.peek() {
        return Ok(*c);
    }
    Err(error.clone())
}

fn load_comment(iter: &mut Peekable<Chars>) -> Result<Token, ParserError> {
    let mut comment_string = String::new();
    let error = ParserError::InvalidComment;
    loop {
        let c = peek_next_char(iter, &error)?;
        match c {
            '\n' => {
                break;
            }
            _ => {
                comment_string.push(c);
                iter.next();
            }
        };
    }
    iter.next();
    Ok(Token::Comment(comment_string))
}

fn load_var_number(iter: &mut Peekable<Chars>) -> Result<Token, ParserError> {
    let mut holder = String::new();
    let error = ParserError::InvalidVarNumber;
    let mut is_var = false;
    let mut is_float = false;
    loop {
        let c = peek_next_char(iter, &error)?;
        match c {
            '_' | 'a'...'z' | 'A'...'Z' => {
                is_var = true;
                holder.push(c);
                iter.next();
            }
            '0'...'9' => {
                holder.push(c);
                iter.next();
            }
            '.' => {
                if is_var || is_float {
                    return Err(error);
                }
                is_float = true;
                holder.push(c);
                iter.next();
            }
            _ => break,
        };
    }
    let token = if is_var {
        Token::Var(holder)
    } else if is_float {
        Token::Float(holder.parse().unwrap())
    } else {
        Token::Integer(holder.parse().unwrap())
    };
    Ok(token)
}

fn combine_token(
    iter: &mut Peekable<Chars>,
    end: char,
    separate: bool,
) -> Result<TokenBody, ParserError> {
    let mut token_body = Vec::new();
    let mut current_token = Vec::new();
    let error = ParserError::InvalidGroup;
    iter.next();
    loop {
        let c = peek_next_char(iter, &error)?;
        if c == end {
            break;
        }
        match c {
            ',' => {
                if separate {
                    update_token_body(&mut token_body, &mut current_token);
                }
                iter.next();
            }
            _ => match_token(c, iter, &mut token_body, &mut current_token)?,
        };
    }
    iter.next();
    update_token_body(&mut token_body, &mut current_token);
    Ok(token_body)
}

fn load_equals(iter: &mut Peekable<Chars>) -> Result<Token, ParserError> {
    let error = ParserError::InvalidAssign;
    let c = peek_next_char(iter, &error)?;
    if c != '=' {
        return Err(error);
    }
    iter.next();
    let error = ParserError::InvalidEqualsGreaterLess;
    let c = peek_next_char(iter, &error)?;
    let token = match c {
        '=' => Token::Equals,
        '>' => Token::EqualsGreater,
        '<' => Token::EqualsLess,
        _ => return Ok(Token::Assign),
    };
    iter.next();
    Ok(token)
}

fn multi_char(
    iter: &mut Peekable<Chars>,
    target: char,
    token_result: Vec<Token>,
    error: ParserError,
) -> Result<Token, ParserError> {
    let mut current_token = None;
    for token in token_result.iter() {
        let c = peek_next_char(iter, &error)?;
        if c != target {
            break;
        }
        current_token = Some(token.clone());
        iter.next();
    }
    if let Some(token) = current_token {
        return Ok(token);
    }
    Err(error)
}
