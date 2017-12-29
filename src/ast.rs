use std::str::Chars;
use std::iter::Peekable;

use super::error::Error;

#[derive(Debug)]
pub enum Ast {
    Comment(String),
    End,
    Var(String),
    Integer(i64),
    // var, args, body
    Function(Box<Ast>, Vec<Vec<Ast>>, Vec<Ast>),
    If(Vec<Ast>),
    Assign,
    Equals,
    Add,
    Sub,
    IoWrite,
    IoAppend,
    Return,
}

impl Ast {
    pub fn is_end(&self) -> bool {
        if let Ast::End = *self {
            return true;
        }
        false
    }
}

pub fn load_ast<'a>(iter: &mut Peekable<Chars>) -> Result<Vec<Ast>, Error<'a>> {
    let mut ast = Vec::new();
    while iter.peek().is_some() {
        if let Some(statement) = load_statement(iter)? {
            ast.push(statement);
        } else {
            iter.next();
        }
    }
    Ok(ast)
}

fn load_statement<'a>(iter: &mut Peekable<Chars>) -> Result<Option<Ast>, Error<'a>> {
    loop {
        let c = match iter.peek() {
            Some(c) => *c,
            None => break,
        };
        match c {
            '\n' => {
                iter.next();
                return Ok(Some(Ast::End));
            }
            ';' => {
                iter.next();
                if let Some(c) = iter.peek() {
                    if *c == ';' {
                        return Ok(Some(Ast::Return));
                    }
                }
                return Ok(Some(Ast::End));
            }
            '.' => {
                iter.next();
                let fn_name = load_var(iter)?;
                let fn_args = load_args(iter)?;
                let body = load_block(iter)?;
                return Ok(Some(Ast::Function(Box::new(fn_name), fn_args, body)));
            }
            '#' => return Ok(Some(load_comment(iter)?)),
            '_' | 'a'...'z' | 'A'...'Z' => return Ok(Some(load_var(iter)?)),
            '0'...'9' => return Ok(Some(load_number(iter)?)),
            '+' => {
                iter.next();
                return Ok(Some(Ast::Add));
            }
            '-' => {
                iter.next();
                return Ok(Some(Ast::Sub));
            }
            '=' => return Ok(Some(load_eqauls(iter)?)),
            '?' => return Ok(Some(Ast::If(load_block(iter)?))),
            '>' => return Ok(Some(load_io_out(iter)?)),
            _ => return Ok(None),
        };
    }
    Ok(None)
}

fn load_args<'a>(iter: &mut Peekable<Chars>) -> Result<Vec<Vec<Ast>>, Error<'a>> {
    // find first ,
    loop {
        let c = match iter.next() {
            Some(c) => c,
            None => return Err(Error::InvalidArgs("No more charaters")),
        };
        if c == ',' {
            break;
        }
    }
    let mut args = Vec::new();
    let mut current_arg = Vec::new();
    loop {
        let c = match iter.peek() {
            Some(c) => *c,
            None => return Err(Error::InvalidArgs("No more charaters")),
        };
        if c == ',' {
            iter.next();
            args.push(current_arg);
            current_arg = Vec::new();
        } else if c == '{' || c == '\n' || c == ';' {
            args.push(current_arg);
            return Ok(args);
        } else {
            if let Some(statement) = load_statement(iter)? {
                current_arg.push(statement);
            } else {
                iter.next();
            }
        }
    }
}

fn load_block<'a>(iter: &mut Peekable<Chars>) -> Result<Vec<Ast>, Error<'a>> {
    let mut ast = Vec::new();
    // get opening block
    loop {
        let c = match iter.next() {
            Some(c) => c,
            None => return Err(Error::InvalidBlock("No more charaters before {")),
        };
        if c == ';' || c == '\n' {
            return Ok(ast);
        }
        if c == '{' {
            break;
        }
    }
    loop {
        let c = match iter.peek() {
            Some(c) => *c,
            None => return Err(Error::InvalidBlock("No more charaters brefore }")),
        };
        if c != '}' {
            if let Some(statement) = load_statement(iter)? {
                ast.push(statement);
            } else {
                iter.next();
            }
        } else {
            iter.next();
            return Ok(ast);
        }
    }
}

fn load_comment<'a>(iter: &mut Peekable<Chars>) -> Result<Ast, Error<'a>> {
    let mut comment = String::new();
    loop {
        let c = match iter.peek() {
            Some(c) => *c,
            None => break,
        };
        match c {
            '\n' => return Ok(Ast::Comment(comment)),
            _ => {
                comment.push(c);
                iter.next();
            }
        };
    }
    Err(Error::InvalidComment("No more charaters"))
}

fn load_var<'a>(iter: &mut Peekable<Chars>) -> Result<Ast, Error<'a>> {
    let mut var = String::new();
    loop {
        let c = match iter.peek() {
            Some(c) => *c,
            None => break,
        };
        match c {
            '_' | 'a'...'z' | 'A'...'Z' | '0'...'9' => {
                var.push(c);
                iter.next();
            }
            _ => return Ok(Ast::Var(var)),
        };
    }
    Err(Error::InvalidVar("No more charaters"))
}

fn load_number<'a>(iter: &mut Peekable<Chars>) -> Result<Ast, Error<'a>> {
    let mut number = String::new();
    loop {
        let c = match iter.peek() {
            Some(c) => *c,
            None => break,
        };
        match c {
            '0'...'9' => {
                number.push(c);
                iter.next();
            }
            _ => {
                let int: i64 = number.parse().unwrap();
                return Ok(Ast::Integer(int));
            }
        };
    }
    Err(Error::InvalidNumber("No more charaters"))
}

fn load_eqauls<'a>(iter: &mut Peekable<Chars>) -> Result<Ast, Error<'a>> {
    let c = match iter.next() {
        Some(c) => c,
        None => return Err(Error::InvalidEquals("No more charaters")),
    };
    if c == '=' {
        let c2 = match iter.peek() {
            Some(c2) => *c2,
            None => return Ok(Ast::Assign),
        };
        if c2 == '=' {
            iter.next();
            return Ok(Ast::Equals);
        }
        return Ok(Ast::Assign);
    }
    Err(Error::InvalidComment("Equal charater mismatch"))
}

fn load_io_out<'a>(iter: &mut Peekable<Chars>) -> Result<Ast, Error<'a>> {
    let c = match iter.next() {
        Some(c) => c,
        None => return Err(Error::InvalidIoWrite("No more charaters")),
    };
    if c == '>' {
        let c2 = match iter.peek() {
            Some(c2) => *c2,
            None => return Ok(Ast::IoWrite),
        };
        if c2 == '>' {
            iter.next();
            return Ok(Ast::IoAppend);
        }
        return Ok(Ast::IoWrite);
    }
    return Err(Error::InvalidIoWrite("Io charater mismatch"));
}
