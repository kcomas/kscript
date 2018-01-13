mod ast;
mod symbol;
mod builder;

use std::str::Chars;
use std::iter::Peekable;
use super::command::Command;
use super::error::ParserError;
use self::ast::{load_ast_til_end, shunt_yard};
pub use self::symbol::SymbolTable;
use self::builder::load_commands_from_ast;
pub use self::ast::Ast;

pub fn build_commands(
    iter: &mut Peekable<Chars>,
    root_symbols: &mut SymbolTable,
) -> Result<Vec<Command>, ParserError> {
    let mut commands = Vec::new();
    while iter.peek().is_some() {
        let mut ast = load_ast_til_end(iter)?;
        if ast.len() > 0 {
            println!("{:?}", ast);
            let shunt = shunt_yard(&mut ast, root_symbols)?;
            println!("{:?}", shunt);
            if shunt.len() > 0 {
                let new_commands = load_commands_from_ast(&shunt)?;
                println!("{:?}", new_commands);
            }
        }
    }
    Ok(commands)
}
