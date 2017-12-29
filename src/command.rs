use std::collections::HashMap;
use super::data_type::DataType;
use super::ast::Ast;
use super::error::Error;

#[derive(Debug)]
pub enum Command {
    // add to stack
    Push(DataType),
    // remove from stack
    Pop,
    // load argument from saved stack position
    LoadArg,
    // save value to save stack
    Save,
    // restore value from save stack
    Restore,
    Equals,
    Sub,
    Add,
    // number of args, function position
    Call(usize, usize),
    Return,
    // exit code
    Halt(usize),
}

#[derive(Debug)]
pub struct SymbolTable {
    // name, index
    functions: HashMap<String, usize>,
    function_counter: usize,
    vars: HashMap<String, usize>,
    var_counter: usize,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            functions: HashMap::new(),
            function_counter: 0,
            vars: HashMap::new(),
            var_counter: 0,
        }
    }
}

pub fn load_commands<'a>(ast: &mut Vec<Ast>) -> Result<Vec<Command>, Error<'a>> {
    let commands: Vec<Command> = Vec::new();
    let mut start_index = 0;
    let mut end_index = 0;
    while start_index < ast.len() - 1 {
        // find the next end token or get to the end of the commands
        while !ast[end_index].is_end() && end_index < ast.len() {
            end_index += 1;
        }
        if start_index < end_index {
            println!("start: {}, end: {}", start_index, end_index);
        }
        end_index += 1;
        start_index = end_index;
    }
    Ok(commands)
}
