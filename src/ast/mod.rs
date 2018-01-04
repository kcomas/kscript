mod ast;

use std::str::Chars;
use std::iter::Peekable;
use super::error::Error;
pub use self::ast::Ast;

struct StopChars {
    chars: Vec<char>,
}

impl StopChars {
    pub fn new(chars: Vec<char>) -> StopChars {
        StopChars { chars: chars }
    }

    pub fn is_stop(&self, c: char) -> bool {
        for stop_char in self.chars.iter() {
            if c == *stop_char {
                return true;
            }
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
    let fn_stop = StopChars::new(vec!['{', '\n', ';']);
    let array_stop = StopChars::new(vec![']']);
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
            '(' => return Ok(Some(Ast::Group(load_block(iter, '(', ')')?))),
            '[' => return Ok(Some(Ast::Access(load_block(iter, '[', ']')?))),
            '.' => {
                iter.next();
                let fn_name = load_var(iter)?;
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
                let fn_args = load_args(iter, &fn_stop)?;
                let body = load_block(iter, '{', '}')?;
                return Ok(Some(Ast::Function(Box::new(fn_name), fn_args, body)));
            }
            '@' => {
                iter.next();
                let c = match iter.next() {
                    Some(c) => c,
                    None => return Err(Error::InvalidArray("No more charaters")),
                };
                let rst;
                match c {
                    '[' => {
                        rst = Some(Ast::Array(load_args(iter, &array_stop)?));
                    }
                    '<' => {
                        rst = Some(Ast::ArrayPush);
                    }
                    _ => return Err(Error::InvalidArray("Invalid array charater")),
                };
                iter.next();
                return Ok(rst);
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
            '*' => {
                iter.next();
                let mut has_exp = false;
                if let Some(c) = iter.peek() {
                    if *c == '*' {
                        has_exp = true;
                    }
                }
                if has_exp {
                    iter.next();
                    return Ok(Some(Ast::Exp));
                } else {
                    return Ok(Some(Ast::Mul));
                }
            }
            '/' => {
                iter.next();
                let mut has_rem = false;
                if let Some(c) = iter.peek() {
                    if *c == '/' {
                        has_rem = true;
                    }
                }
                if has_rem {
                    iter.next();
                    return Ok(Some(Ast::Rem));
                } else {
                    return Ok(Some(Ast::Div));
                }
            }
            '=' => return Ok(Some(load_eqauls(iter)?)),
            '?' => return Ok(Some(Ast::If(load_block(iter, '{', '}')?))),
            '>' => return Ok(Some(load_io_out(iter)?)),
            '"' => return Ok(Some(load_string(iter)?)),
            _ => return Ok(None),
        };
    }
    Ok(None)
}

fn load_string<'a>(iter: &mut Peekable<Chars>) -> Result<Ast, Error<'a>> {
    // find first "
    loop {
        let c = match iter.next() {
            Some(c) => c,
            None => return Err(Error::InvalidString("No more charaters")),
        };
        if c == '"' {
            break;
        }
    }
    let mut string = String::new();
    loop {
        let c = match iter.peek() {
            Some(c) => *c,
            None => break,
        };
        match c {
            '"' => {
                iter.next();
                return Ok(Ast::String(string));
            }
            '\\' => {
                iter.next();
                let next = match iter.next() {
                    Some(n) => n,
                    None => return Err(Error::InvalidString("No more charaters for escape")),
                };
                match next {
                    '\\' => string.push('\\'),
                    'n' => string.push('\n'),
                    '"' => string.push('"'),
                    _ => return Err(Error::InvalidString("Invalid escape")),
                };
            }
            _ => {
                string.push(c);
                iter.next();
            }
        }
    }
    Err(Error::InvalidString("No more chraters"))
}

fn load_args<'a>(
    iter: &mut Peekable<Chars>,
    stop_chars: &StopChars,
) -> Result<Vec<Vec<Ast>>, Error<'a>> {
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
        } else if stop_chars.is_stop(c) {
            if current_arg.len() > 0 {
                args.push(current_arg);
            }
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

fn load_block<'a>(
    iter: &mut Peekable<Chars>,
    start: char,
    end: char,
) -> Result<Vec<Ast>, Error<'a>> {
    let mut ast = Vec::new();
    // get opening block
    loop {
        let c = match iter.next() {
            Some(c) => c,
            None => return Err(Error::InvalidBlock("No more charaters before start")),
        };
        if c == ';' || c == '\n' {
            return Ok(ast);
        }
        if c == start {
            break;
        }
    }
    loop {
        let c = match iter.peek() {
            Some(c) => *c,
            None => return Err(Error::InvalidBlock("No more charaters brefore end")),
        };
        if c != end {
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
            _ => {
                if var == "t" {
                    return Ok(Ast::Bool(true));
                } else if var == "f" {
                    return Ok(Ast::Bool(false));
                }
                return Ok(Ast::Var(var));
            }
        };
    }
    Err(Error::InvalidVar("No more charaters"))
}

fn load_number<'a>(iter: &mut Peekable<Chars>) -> Result<Ast, Error<'a>> {
    let mut number = String::new();
    let mut is_float = false;
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
            '.' => {
                if !is_float {
                    number.push(c);
                    iter.next();
                    is_float = true;
                } else {
                    return Err(Error::InvalidNumber("Came across another ."));
                }
            }
            _ => {
                if !is_float {
                    let int: i64 = number.parse().unwrap();
                    return Ok(Ast::Integer(int));
                } else {
                    let float: f64 = number.parse().unwrap();
                    return Ok(Ast::Float(float));
                }
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
