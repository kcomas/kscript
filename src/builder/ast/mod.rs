mod ast;

use std::str::Chars;
use std::iter::Peekable;
use self::ast::Ast;
use super::super::error::ParserError;

pub fn load_ast_til_end(iter: &mut Peekable<Chars>) -> Result<Vec<Ast>, ParserError> {
    let mut ast = Vec::new();
    while let Some(item) = match_ast(iter)? {
        ast.push(item);
    }
    Ok(ast)
}

fn match_ast(iter: &mut Peekable<Chars>) -> Result<Option<Ast>, ParserError> {
    loop {
        let c = match iter.peek() {
            Some(c) => *c,
            None => break,
        };
        match c {
            '\n' => {
                iter.next();
                return Ok(None);
            }
            '#' => return Ok(Some(load_comment(iter)?)),
            ';' => {
                iter.next();
                let c = match iter.peek() {
                    Some(c) => *c,
                    None => return Ok(None),
                };
                if c == ';' {
                    iter.next();
                    return Ok(Some(Ast::Return));
                }
                return Ok(None);
            }
            'a'...'z' | 'A'...'Z' => return Ok(Some(load_var(iter)?)),
            _ => iter.next(),
        };
    }
    Ok(None)
}

fn peek_next_char(iter: &mut Peekable<Chars>, error: &ParserError) -> Result<char, ParserError> {
    match iter.peek() {
        Some(c) => Ok(*c),
        None => Err(error.clone()),
    }
}

fn load_var(iter: &mut Peekable<Chars>) -> Result<Ast, ParserError> {
    let mut var = String::new();
    let error = ParserError::InvalidVar;
    loop {
        let c = peek_next_char(iter, &error)?;
        match c {
            '_' | 'a'...'z' | 'A'...'Z' | '0'...'9' => {
                var.push(c);
                iter.next();
            }
            _ => break,
        }
    }
    if var == "t" {
        return Ok(Ast::Bool(true));
    } else if var == "f" {
        return Ok(Ast::Bool(false));
    }
    Ok(Ast::Var(var))
}

fn load_comment(iter: &mut Peekable<Chars>) -> Result<Ast, ParserError> {
    let mut comment = String::new();
    let error = ParserError::InvalidComment;
    loop {
        let c = peek_next_char(iter, &error)?;
        match c {
            '\n' => break,
            _ => {
                comment.push(c);
                iter.next();
            }
        };
    }
    Ok(Ast::Comment(comment))
}
