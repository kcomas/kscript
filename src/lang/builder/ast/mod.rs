mod ast;
mod jump;

use std::str::Chars;
use std::iter::Peekable;
pub use self::ast::{Ast, AstArgs, AstBody};
use super::super::error::ParserError;
pub use self::jump::shunt_yard;

pub fn load_ast_til_end(iter: &mut Peekable<Chars>) -> Result<Vec<Ast>, ParserError> {
    let mut ast = Vec::new();
    'out: loop {
        while let Some(item) = match_ast(iter)? {
            if let Ast::End = item {
                break 'out;
            }
            let add_end = item.add_end();
            ast.push(item);
            if add_end {
                break 'out;
            }
        }
        if iter.next().is_none() {
            break;
        }
    }
    Ok(ast)
}

fn match_ast(iter: &mut Peekable<Chars>) -> Result<Option<Ast>, ParserError> {
    let c = match iter.peek() {
        Some(c) => *c,
        None => return Ok(None),
    };
    match c {
        '\n' => {
            iter.next();
            return Ok(Some(Ast::End));
        }
        '#' => return Ok(Some(load_comment(iter)?)),
        ';' => {
            iter.next();
            let c = match iter.peek() {
                Some(c) => *c,
                None => return Ok(Some(Ast::End)),
            };
            if c == ';' {
                return Ok(Some(Ast::Return));
            }
            return Ok(Some(Ast::End));
        }
        'a'...'z' | 'A'...'Z' => return Ok(Some(load_var(iter)?)),
        '0'...'9' => return Ok(Some(load_number(iter)?)),
        '.' => {
            iter.next();
            let (args, end_c) = load_items(iter, "\n;{")?;
            if end_c == '{' {
                return Ok(Some(Ast::Function(args, load_block(iter, '{', '}')?)));
            }
            if end_c == ';' {
                iter.next();
            }
            return Ok(Some(Ast::FunctionCall(args)));
        }
        '(' => return Ok(Some(Ast::Group(load_block(iter, '(', ')')?))),
        '"' => return Ok(Some(load_string(iter)?)),
        '?' => return Ok(Some(Ast::If(load_block(iter, '{', '}')?))),
        '=' => return Ok(Some(load_equals(iter)?)),
        '+' => {
            return Ok(Some(double_char(
                iter,
                '+',
                (ParserError::InvalidAdd, ParserError::InvalidConcat),
                (Ast::Add, Ast::Concat),
            )?))
        }
        '-' => return Ok(Some(next_and_return(iter, Ast::Sub))),
        '*' => {
            return Ok(Some(double_char(
                iter,
                '*',
                (ParserError::InvalidMul, ParserError::InvalidExp),
                (Ast::Mul, Ast::Exp),
            )?))
        }
        '/' => {
            return Ok(Some(double_char(
                iter,
                '/',
                (ParserError::InvalidDiv, ParserError::InvalidRem),
                (Ast::Div, Ast::Rem),
            )?))
        }
        '>' => {
            return Ok(Some(double_char(
                iter,
                '>',
                (ParserError::InvalidIoWrite, ParserError::InvalidIoAppend),
                (Ast::IoWrite, Ast::IoAppend),
            )?))
        }
        _ => return Ok(None),
    };
}

fn next_and_return(iter: &mut Peekable<Chars>, ast: Ast) -> Ast {
    iter.next();
    ast
}

fn peek_next_char(iter: &mut Peekable<Chars>, error: &ParserError) -> Result<char, ParserError> {
    match iter.peek() {
        Some(c) => Ok(*c),
        None => Err(error.clone()),
    }
}

fn load_block(iter: &mut Peekable<Chars>, start: char, end: char) -> Result<AstBody, ParserError> {
    let mut ast = Vec::new();
    let mut current_ast = Vec::new();
    let error = ParserError::InvalidBlockStart;
    loop {
        let c = peek_next_char(iter, &error)?;
        if c == start {
            iter.next();
            break;
        }
        iter.next();
    }
    let error = ParserError::InvalidBlock;
    loop {
        let c = peek_next_char(iter, &error)?;
        if c == end {
            if current_ast.len() > 0 {
                ast.push(current_ast);
            }
            iter.next();
            break;
        }
        if let Some(statement) = match_ast(iter)? {
            if let Ast::End = statement {
                if current_ast.len() > 0 {
                    ast.push(current_ast);
                    current_ast = Vec::new();
                }
            } else {
                current_ast.push(statement);
            }
        } else {
            iter.next();
        }
    }
    Ok(ast)
}

fn load_items(
    iter: &mut Peekable<Chars>,
    stop_chars: &str,
) -> Result<(AstArgs, char), ParserError> {
    let mut args = Vec::new();
    let mut current_arg = Vec::new();
    let mut current_statements = Vec::new();
    let error = ParserError::InvalidItem;
    let mut c;
    'out: loop {
        c = peek_next_char(iter, &error)?;
        for stop_char in stop_chars.chars() {
            if c == stop_char {
                if current_statements.len() > 0 {
                    current_arg.push(current_statements);
                }
                if current_arg.len() > 0 {
                    args.push(current_arg);
                }
                break 'out;
            }
        }
        if c == ',' {
            iter.next();
            if current_statements.len() > 0 {
                current_arg.push(current_statements);
                current_statements = Vec::new();
            }
            args.push(current_arg);
            current_arg = Vec::new();
        } else {
            if let Some(statement) = match_ast(iter)? {
                if let Ast::End = statement {
                    current_arg.push(current_statements);
                    current_statements = Vec::new();
                } else {
                    current_statements.push(statement);
                }
            } else {
                iter.next();
            }
        }
    }
    Ok((args, c))
}

fn load_til_end(iter: &mut Peekable<Chars>, stop_chars: &str) -> Result<AstBody, ParserError> {
    let mut ast = Vec::new();
    let mut current_statements = Vec::new();
    let error = ParserError::InvalidPart;
    let mut c;
    'out: loop {
        c = peek_next_char(iter, &error)?;
        for stop_char in stop_chars.chars() {
            if c == stop_char {
                if current_statements.len() > 0 {
                    ast.push(current_statements);
                }
                break 'out;
            }
        }
        if let Some(statement) = match_ast(iter)? {
            if let Ast::End = statement {
                ast.push(current_statements);
                current_statements = Vec::new();
            } else {
                current_statements.push(statement);
            }
        } else {
            iter.next();
        }
    }
    Ok(ast)
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

fn load_number(iter: &mut Peekable<Chars>) -> Result<Ast, ParserError> {
    let mut number = String::new();
    let mut is_float = false;
    let error = ParserError::InvalidNumber;
    loop {
        let c = peek_next_char(iter, &error)?;
        match c {
            '0'...'9' => {
                number.push(c);
                iter.next();
            }
            '.' => {
                if is_float {
                    return Err(ParserError::InvalidFloat);
                }
                number.push(c);
                iter.next();
                is_float = true;
            }
            _ => break,
        }
    }
    match is_float {
        true => Ok(Ast::Float(number.parse().unwrap())),
        false => Ok(Ast::Integer(number.parse().unwrap())),
    }
}

fn load_string(iter: &mut Peekable<Chars>) -> Result<Ast, ParserError> {
    let error = ParserError::InvalidStringStart;
    let c = peek_next_char(iter, &error)?;
    if c == '"' {
        iter.next();
    } else {
        return Err(error);
    }
    let mut string = String::new();
    let error = ParserError::InvalidString;
    loop {
        let c = peek_next_char(iter, &error)?;
        match c {
            '"' => {
                iter.next();
                return Ok(Ast::String(string));
            }
            '\\' => {
                iter.next();
                let error = ParserError::InvalidStringEscape;
                let c2 = peek_next_char(iter, &error)?;
                match c2 {
                    '\\' => string.push('\\'),
                    't' => string.push('\t'),
                    'n' => string.push('\n'),
                    _ => return Err(error),
                };
                iter.next();
            }
            _ => {
                string.push(c);
                iter.next();
            }
        }
    }
}

fn double_char(
    iter: &mut Peekable<Chars>,
    to_match: char,
    error_types: (ParserError, ParserError),
    return_types: (Ast, Ast),
) -> Result<Ast, ParserError> {
    let c = peek_next_char(iter, &error_types.0)?;
    if c == to_match {
        iter.next();
    } else {
        return Err(error_types.0);
    }
    let c2 = peek_next_char(iter, &error_types.1)?;
    if c2 == to_match {
        iter.next();
        return Ok(return_types.1);
    }
    Ok(return_types.0)
}

fn load_equals(iter: &mut Peekable<Chars>) -> Result<Ast, ParserError> {
    let error = ParserError::InvalidAssign;
    let c = peek_next_char(iter, &error)?;
    if c == '=' {
        iter.next();
    } else {
        return Err(error);
    }
    let error = ParserError::InvalidEquals;
    let c2 = peek_next_char(iter, &error)?;
    if c2 == '=' {
        iter.next();
        return Ok(Ast::Equals);
    }
    Ok(Ast::Assign(load_til_end(iter, "\n;")?))
}
