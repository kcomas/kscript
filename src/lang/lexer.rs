use std::iter::Peekable;
use std::str::Chars;
use super::error::ParserError;
use super::ast::{Ast, AstBody};

pub fn string_to_ast(iter: &mut Peekable<Chars>) -> Result<AstBody, ParserError> {
    let mut ast_body = Vec::new();
    let mut current_ast = Vec::new();

    loop {
        let c = match iter.peek() {
            Some(c) => *c,
            None => break,
        };
        match_ast(c, iter, &mut ast_body, &mut current_ast)?;
    }
    update_ast_body(&mut ast_body, &mut current_ast);
    Ok(ast_body)
}

fn match_ast(
    c: char,
    iter: &mut Peekable<Chars>,
    ast_body: &mut AstBody,
    current_ast: &mut Vec<Ast>,
) -> Result<(), ParserError> {
    match c {
        '\n' | ',' => {
            iter.next();
            update_ast_body(ast_body, current_ast);
        }
        ';' => {
            iter.next();
            let c2 = match iter.peek() {
                Some(c) => *c,
                None => return Err(ParserError::InvalidReturn),
            };
            if c2 == ';' {
                iter.next();
                current_ast.push(Ast::Return);
            }
            update_ast_body(ast_body, current_ast);
        }
        '0'...'9' | '_' | 'a'...'z' | 'A'...'Z' => current_ast.push(load_var_number(iter)?),
        '#' => {
            current_ast.push(load_comment(iter)?);
            update_ast_body(ast_body, current_ast);
        }
        '?' | '+' | '-' | '.' => {
            let token = match c {
                '?' => Ast::If,
                '+' => Ast::Add,
                '-' => Ast::Sub,
                '.' => Ast::Call,
                _ => panic!("Impossible Symbol Mismatch"),
            };
            current_ast.push(token);
            iter.next();
        }
        '=' => current_ast.push(load_equals(iter)?),
        '>' => current_ast.push(multi_char(
            iter,
            '>',
            vec![Ast::Greater, Ast::IoWrite, Ast::IoAppend],
            ParserError::InvalidIoWriteIoAppendGreater,
        )?),
        '(' => current_ast.push(Ast::Group(combine_ast(iter, ')', true)?)),
        '{' => {
            current_ast.push(Ast::Block(combine_ast(iter, '}', false)?));
            update_ast_body(ast_body, current_ast);
        }
        _ => {
            iter.next();
        }
    };
    Ok(())
}

fn update_ast_body(ast_body: &mut AstBody, current_ast: &mut Vec<Ast>) {
    ast_body.push(current_ast.clone());
    current_ast.clear();
}

fn peek_next_char(iter: &mut Peekable<Chars>, error: &ParserError) -> Result<char, ParserError> {
    if let Some(c) = iter.peek() {
        return Ok(*c);
    }
    Err(error.clone())
}

fn load_comment(iter: &mut Peekable<Chars>) -> Result<Ast, ParserError> {
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
    Ok(Ast::Comment(comment_string))
}

fn load_var_number(iter: &mut Peekable<Chars>) -> Result<Ast, ParserError> {
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
        Ast::Var(holder)
    } else if is_float {
        Ast::Float(holder.parse().unwrap())
    } else {
        Ast::Integer(holder.parse().unwrap())
    };
    Ok(token)
}

fn combine_ast(
    iter: &mut Peekable<Chars>,
    end: char,
    separate: bool,
) -> Result<AstBody, ParserError> {
    let mut ast_body = Vec::new();
    let mut current_ast = Vec::new();
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
                    update_ast_body(&mut ast_body, &mut current_ast);
                }
                iter.next();
            }
            _ => match_ast(c, iter, &mut ast_body, &mut current_ast)?,
        };
    }
    iter.next();
    update_ast_body(&mut ast_body, &mut current_ast);
    Ok(ast_body)
}

fn load_equals(iter: &mut Peekable<Chars>) -> Result<Ast, ParserError> {
    let error = ParserError::InvalidAssign;
    let c = peek_next_char(iter, &error)?;
    if c != '=' {
        return Err(error);
    }
    iter.next();
    let error = ParserError::InvalidEqualsGreaterLess;
    let c = peek_next_char(iter, &error)?;
    let token = match c {
        '=' => Ast::Equals,
        '>' => Ast::EqualsGreater,
        '<' => Ast::EqualsLess,
        _ => return Ok(Ast::Assign),
    };
    iter.next();
    Ok(token)
}

fn multi_char(
    iter: &mut Peekable<Chars>,
    target: char,
    ast_result: Vec<Ast>,
    error: ParserError,
) -> Result<Ast, ParserError> {
    let mut current_ast = None;
    for ast in ast_result.iter() {
        let c = peek_next_char(iter, &error)?;
        if c != target {
            break;
        }
        current_ast = Some(ast.clone());
        iter.next();
    }
    if let Some(ast) = current_ast {
        return Ok(ast);
    }
    Err(error)
}
