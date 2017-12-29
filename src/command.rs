use super::data_type::DataType;
use super::ast::Ast;
use super::error::Error;
use super::symbol::SymbolTable;

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

pub fn load_commands<'a>(
    ast: &mut Vec<Ast>,
    commands: &mut Vec<Command>,
    symbols: &mut SymbolTable,
) -> Result<(), Error<'a>> {
    let mut start_index = 0;
    let mut end_index = 0;
    while start_index < ast.len() - 1 {
        // find the next end token or get to the end of the commands
        while !ast[end_index].is_end() && end_index < ast.len() {
            end_index += 1;
        }
        if start_index < end_index {
            println!("start: {}, end: {}", start_index, end_index);
            let mut current_index = start_index;
            let mut highest_presedence = 0;
            let mut highest_presedence_index = 0;
            loop {
                while current_index < end_index {
                    let current_index_presedence = ast[current_index].presedence();
                    if current_index_presedence > highest_presedence {
                        highest_presedence = current_index_presedence;
                        highest_presedence_index = current_index;
                    }
                    current_index += 1;
                }
                if highest_presedence == 0 {
                    break;
                }
                // work with this token
                println!("{:?}", ast[highest_presedence_index]);
                if ast[highest_presedence_index].is_function() {
                    if ast[highest_presedence_index].is_function_def() {
                        let fn_name = ast[highest_presedence_index].get_function_name();
                        symbols.register_function(fn_name, commands.len())?;
                    } else {

                    }
                }
                ast[highest_presedence_index] = Ast::Used;
                // reset
                current_index = start_index;
                highest_presedence = 0;
                highest_presedence_index = 0
            }
        }
        end_index += 1;
        start_index = end_index;
    }
    Ok(())
}
