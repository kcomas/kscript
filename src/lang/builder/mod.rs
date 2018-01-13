mod ast;
mod symbol;
mod builder;

use std::str::Chars;
use std::iter::Peekable;
use std::rc::Rc;
use super::command::{Command, SharedCommands};
use super::error::ParserError;
use self::ast::{load_ast_til_end, shunt_yard};
pub use self::symbol::SymbolTable;
use self::builder::load_commands_from_ast;
use self::ast::AstBody;
pub use self::ast::Ast;
use super::util::{write_debug, KscriptDebug};

pub fn build_commands(
    iter: &mut Peekable<Chars>,
    root_symbols: &mut SymbolTable,
    debug: &Option<KscriptDebug>,
) -> Result<SharedCommands, ParserError> {
    if debug.is_some() {
        return build_debug(iter, root_symbols, debug);
    }
    build(iter, root_symbols)
}

fn build_debug(
    iter: &mut Peekable<Chars>,
    root_symbols: &mut SymbolTable,
    debug: &Option<KscriptDebug>,
) -> Result<SharedCommands, ParserError> {
    let mut debug_ast = Vec::new();
    let mut debug_shunt_ast = Vec::new();
    let mut commands = Vec::new();

    let print_debug = |debug_ast: &AstBody, debug_shunt_ast: &AstBody, commands: &Vec<Command>| {
        write_debug("Ast", &format!("{:#?}", debug_ast), debug).unwrap();
        write_debug("Shunted Ast", &format!("{:#?}", debug_shunt_ast), debug).unwrap();
        write_debug("Commands", &format!("{:#?}", commands), debug).unwrap();
    };

    while iter.peek().is_some() {
        let mut ast = match load_ast_til_end(iter) {
            Ok(ast) => ast,
            Err(error) => {
                print_debug(&debug_ast, &debug_shunt_ast, &commands);
                return Err(error);
            }
        };
        debug_ast.push(ast.clone());
        if ast.len() > 0 {
            let shunt = match shunt_yard(&mut ast, root_symbols) {
                Ok(shunt) => shunt,
                Err(error) => {
                    print_debug(&debug_ast, &debug_shunt_ast, &commands);
                    return Err(error);
                }
            };
            debug_shunt_ast.push(shunt.clone());
            if shunt.len() > 0 {
                let mut new_commands = match load_commands_from_ast(&shunt) {
                    Ok(new_commands) => new_commands,
                    Err(error) => {
                        print_debug(&debug_ast, &debug_shunt_ast, &commands);
                        return Err(error);
                    }
                };
                commands.append(&mut new_commands);
            }
        }
    }
    commands.push(Command::Halt(0));
    print_debug(&debug_ast, &debug_shunt_ast, &commands);
    Ok(Rc::new(commands))
}

fn build(
    iter: &mut Peekable<Chars>,
    root_symbols: &mut SymbolTable,
) -> Result<SharedCommands, ParserError> {
    let mut commands = Vec::new();
    while iter.peek().is_some() {
        let mut ast = load_ast_til_end(iter)?;
        if ast.len() > 0 {
            let shunt = shunt_yard(&mut ast, root_symbols)?;
            if shunt.len() > 0 {
                let mut new_commands = load_commands_from_ast(&shunt)?;
                commands.append(&mut new_commands);
            }
        }
    }
    commands.push(Command::Halt(0));
    Ok(Rc::new(commands))
}
