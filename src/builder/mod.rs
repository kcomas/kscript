mod ast;

use std::str::Chars;
use std::iter::Peekable;
use super::command::Command;
use super::error::ParserError;
use self::ast::load_ast_til_end;

pub fn build_commands(iter: &mut Peekable<Chars>) -> Result<Vec<Command>, ParserError> {
    while iter.peek().is_some() {
        let ast = load_ast_til_end(iter)?;
        println!("{:?}", ast);
    }
    Ok(Vec::new())
}
