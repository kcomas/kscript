mod ast;

use std::str::Chars;
use std::iter::Peekable;
use self::ast::Ast;
use super::super::error::ParserError;

pub fn load_ast_til_end(iter: &mut Peekable<Chars>) -> Result<Vec<Ast>, ParserError> {
    let mut ast = Vec::new();
    loop {
        while let Some(item) = match_ast(iter)? {
            if let Ast::End = item {
                return Ok(ast);
            }
            ast.push(item);
        }
        iter.next();
    }
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
        '?' => return Ok(Some(Ast::If(load_block(iter, '{', '}')?))),
        '=' => return Ok(Some(load_equals(iter)?)),
        '+' => return Ok(Some(next_and_return(iter, Ast::Add))),
        '-' => return Ok(Some(next_and_return(iter, Ast::Sub))),
        '>' => return Ok(Some(load_io_out(iter)?)),
        _ => return Ok(None),
    };
    Ok(None)
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

fn load_block(
    iter: &mut Peekable<Chars>,
    start: char,
    end: char,
) -> Result<Vec<Vec<Ast>>, ParserError> {
    let mut ast = Vec::new();
    let mut current_ast = Vec::new();
    let error = ParserError::InvalidBlockStart;
    loop {
        let c = peek_next_char(iter, &error)?;
        if c == start {
            break;
        }
        iter.next();
    }
    let error = ParserError::InvalidBlock;
    loop {
        let c = peek_next_char(iter, &error)?;
        if c == end {
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
) -> Result<(Vec<Vec<Vec<Ast>>>, char), ParserError> {
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
    Ok(Ast::Assign)
}

fn load_io_out(iter: &mut Peekable<Chars>) -> Result<Ast, ParserError> {
    let error = ParserError::InvalidIoWrite;
    let c = peek_next_char(iter, &error)?;
    if c == '>' {
        iter.next();
    } else {
        return Err(error);
    }
    let error = ParserError::InvalidIoAppend;
    let c2 = peek_next_char(iter, &error)?;
    if c2 == '>' {
        iter.next();
        return Ok(Ast::IoAppend);
    }
    Ok(Ast::IoWrite)
}
