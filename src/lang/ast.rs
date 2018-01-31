use std::iter::Peekable;
use std::str::Chars;
use super::error::ParserError;

pub type AstBody = Vec<Vec<Ast>>;

#[derive(Debug, Clone)]
pub enum Ast {
    Comment(String),
    Integer(i64),
    Float(f64),
    Var(String),
    Group(AstBody),
    FunctionCall(String, AstBody),
    Return,
}

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
        '(' => current_ast.push(Ast::Group(load_group(iter)?)),
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
            '(' => {
                if !is_var {
                    return Err(ParserError::InvalidFunctionCall);
                }
                return Ok(Ast::FunctionCall(holder, load_group(iter)?));
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

fn load_group(iter: &mut Peekable<Chars>) -> Result<AstBody, ParserError> {
    let mut ast_body = Vec::new();
    let mut current_ast = Vec::new();
    let error = ParserError::InvalidGroup;
    iter.next();
    loop {
        let c = peek_next_char(iter, &error)?;
        match c {
            ',' => {
                update_ast_body(&mut ast_body, &mut current_ast);
                iter.next();
            }
            ')' => break,
            _ => match_ast(c, iter, &mut ast_body, &mut current_ast)?,
        };
    }
    iter.next();
    update_ast_body(&mut ast_body, &mut current_ast);
    Ok(ast_body)
}
