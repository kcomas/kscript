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
use self::ast::AstBody;
pub use self::ast::Ast;
use super::util::{write_debug, KscriptDebug};
use super::function::FunctionLookup;

pub fn build_commands(
    iter: &mut Peekable<Chars>,
    root_symbols: &mut SymbolTable,
    functions: &mut FunctionLookup,
    debug: &Option<KscriptDebug>,
) -> Result<(), ParserError> {
    if debug.is_some() {
        return build_debug(iter, root_symbols, functions, debug);
    }
    build(iter, root_symbols, functions)
}

fn build_debug(
    iter: &mut Peekable<Chars>,
    root_symbols: &mut SymbolTable,
    functions: &mut FunctionLookup,
    debug: &Option<KscriptDebug>,
) -> Result<(), ParserError> {
    let mut debug_ast = Vec::new();
    let mut debug_shunt_ast = Vec::new();

    let print_debug =
        |debug_ast: &AstBody, debug_shunt_ast: &AstBody, functions: &FunctionLookup| {
            write_debug("Ast", &format!("{:#?}", debug_ast), debug).unwrap();
            write_debug("Shunted Ast", &format!("{:#?}", debug_shunt_ast), debug).unwrap();
            write_debug("Functions", &format!("{:#?}", functions), debug).unwrap();
        };

    while iter.peek().is_some() {
        let mut ast = match load_ast_til_end(iter) {
            Ok(ast) => ast,
            Err(error) => {
                print_debug(&debug_ast, &debug_shunt_ast, functions);
                return Err(error);
            }
        };
        debug_ast.push(ast.clone());
        if ast.len() > 0 {
            let shunt = match shunt_yard(&mut ast, root_symbols) {
                Ok(shunt) => shunt,
                Err(error) => {
                    print_debug(&debug_ast, &debug_shunt_ast, functions);
                    return Err(error);
                }
            };
            debug_shunt_ast.push(shunt.clone());
            if shunt.len() > 0 {
                let new_commands = match load_commands_from_ast(&shunt, functions) {
                    Ok(new_commands) => new_commands,
                    Err(error) => {
                        print_debug(&debug_ast, &debug_shunt_ast, functions);
                        return Err(error);
                    }
                };
                functions.update(new_commands, 0);
            }
        }
    }
    functions.push(Command::Halt(0), 0);
    print_debug(&debug_ast, &debug_shunt_ast, functions);
    Ok(())
}

fn build(
    iter: &mut Peekable<Chars>,
    root_symbols: &mut SymbolTable,
    functions: &mut FunctionLookup,
) -> Result<(), ParserError> {
    while iter.peek().is_some() {
        let mut ast = load_ast_til_end(iter)?;
        if ast.len() > 0 {
            let shunt = shunt_yard(&mut ast, root_symbols)?;
            if shunt.len() > 0 {
                let new_commands = load_commands_from_ast(&shunt, functions)?;
                functions.update(new_commands, 0);
            }
        }
    }
    functions.push(Command::Halt(0), 0);
    Ok(())
}
