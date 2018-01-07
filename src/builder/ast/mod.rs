mod ast;

use std::str::Chars;
use std::iter::Peekable;
use self::ast::Ast;
use super::super::error::ParserError;

pub fn load_ast_til_end(iter: &mut Peekable<Chars>) -> Result<Vec<Ast>, ParserError> {
    let mut ast = Vec::new();
    Ok(ast)
}

fn match_ast(iter: &mut Peekable<Chars>) -> Result<Option<Ast>, ParserError> {
    Ok(None)
}
