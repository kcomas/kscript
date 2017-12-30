use super::data_type::DataType;
use super::ast::Ast;
use super::error::Error;
use super::symbol::SymbolTable;

#[derive(Debug)]
pub enum Command {
    // add to stack
    Push(DataType),
    // remove from stack
    // Pop,
    // load argument from the locals stack
    Load(usize),
    // save value to locals stack
    // Save(usize)
    Equals,
    Add,
    Sub,
    IoWrite,
    IoAppend,
    // jump to position if false
    Jmpf(usize),
    // number of args, function position
    Call(usize, usize),
    Return,
    // exit code
    Halt(usize),
}

impl Command {
    pub fn is_return(&self) -> bool {
        if let Command::Return = *self {
            return true;
        }
        false
    }

    pub fn is_halt(&self) -> bool {
        if let Command::Halt(_) = *self {
            return true;
        }
        false
    }
}

pub fn load_commands<'a>(
    ast: &mut Vec<Ast>,
    commands: &mut Vec<Command>,
    symbols: &mut SymbolTable,
) -> Result<(), Error<'a>> {
    let mut start_index = 0;
    let mut end_index = 0;
    while start_index < ast.len() {
        // find the next end token or get to the end of the commands
        while end_index < ast.len() && !ast[end_index].is_end() {
            end_index += 1;
        }
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
            // println!("{:?}", ast[highest_presedence_index]);
            if ast[highest_presedence_index].is_function() {
                if ast[highest_presedence_index].is_function_def() {
                    let add_main_halt;
                    {
                        let fn_name = ast[highest_presedence_index].get_function_name()?;
                        add_main_halt = symbols.register_function(fn_name, commands.len())?;
                    }
                    let mut function_symbol_table = symbols.get_sub_table();
                    {
                        let args = ast[highest_presedence_index].get_function_args()?;
                        // convert the args to indexes
                        for arg in args.iter() {
                            if let Some(var) = arg.get(0) {
                                let var_name = var.get_var_name()?;
                                function_symbol_table.register_var(var_name)?;
                            }
                        }
                    }
                    let fn_body = ast[highest_presedence_index].get_function_body_mut()?;
                    load_commands(fn_body, commands, &mut function_symbol_table)?;
                    // add a halt or return if needed
                    if add_main_halt {
                        let mut add_halt = false;
                        if let Some(cmd) = commands.last() {
                            add_halt = !cmd.is_halt();
                        }
                        if add_halt {
                            commands.push(Command::Halt(0));
                        }
                    } else {
                        let mut add_return = false;
                        if let Some(cmd) = commands.last() {
                            add_return = !cmd.is_return();
                        }
                        if add_return {
                            commands.push(Command::Return);
                        }
                    }
                } else {
                    // function call
                    build_function_call(ast, highest_presedence_index, commands, symbols)?;
                }
            } else if ast[highest_presedence_index].is_if() {
                let mut if_commands = Vec::new();
                load_commands(
                    ast[highest_presedence_index].get_if_body_mut()?,
                    &mut if_commands,
                    symbols,
                )?;
                let jmpf = Command::Jmpf(commands.len() + if_commands.len() + 1);
                commands.push(jmpf);
                commands.append(&mut if_commands);
            } else {
                add_commands(ast, highest_presedence_index, commands, symbols)?;
            }
            ast[highest_presedence_index] = Ast::Used;
            // reset
            current_index = start_index;
            highest_presedence = 0;
            highest_presedence_index = 0
        }
        end_index += 1;
        start_index = end_index;
    }
    Ok(())
}

fn build_function_call<'a>(
    ast: &mut Vec<Ast>,
    index: usize,
    commands: &mut Vec<Command>,
    symbols: &mut SymbolTable,
) -> Result<(), Error<'a>> {
    let fn_index = symbols.get_function_index(ast[index].get_function_name()?)?;
    let args = ast[index].get_function_args_mut()?;
    for arg in args.iter_mut() {
        load_commands(arg, commands, symbols)?;
    }
    commands.push(Command::Call(args.len(), fn_index));
    Ok(())
}

fn add_commands<'a>(
    ast: &mut Vec<Ast>,
    index: usize,
    commands: &mut Vec<Command>,
    symbols: &mut SymbolTable,
) -> Result<(), Error<'a>> {
    if ast[index].is_data() {
        commands.push(transform_command(&ast[index], symbols)?);
        return Ok(());
    }
    if ast[index].is_dyadic() {
        if index > 0 {
            build_command(ast, index - 1, commands, symbols)?;
        }
        if index < ast.len() {
            build_command(ast, index + 1, commands, symbols)?;
        }
    }
    if ast[index].is_monadic() {
        if index > 0 {
            build_command(ast, index - 1, commands, symbols)?;
        }
    }
    let command = match ast[index] {
        Ast::Equals => Command::Equals,
        Ast::Add => Command::Add,
        Ast::Sub => Command::Sub,
        Ast::IoWrite => Command::IoWrite,
        Ast::IoAppend => Command::IoAppend,
        Ast::Return => Command::Return,
        _ => {
            return Err(Error::InvalidAstForCommand(
                ast[index].clone(),
                "Cannot convert to command",
            ))
        }
    };
    commands.push(command);
    Ok(())
}

fn build_command<'a>(
    ast: &mut Vec<Ast>,
    next_index: usize,
    commands: &mut Vec<Command>,
    symbols: &mut SymbolTable,
) -> Result<(), Error<'a>> {
    if !ast[next_index].is_used() {
        if ast[next_index].is_function() {
            build_function_call(ast, next_index, commands, symbols)?;
        } else {
            commands.push(transform_command(&ast[next_index], symbols)?);
        }
        ast[next_index] = Ast::Used;
    }
    Ok(())
}

fn transform_command<'a>(ast: &Ast, symbols: &SymbolTable) -> Result<Command, Error<'a>> {
    if ast.is_var() {
        return Ok(Command::Load(symbols.get_var_index(ast.get_var_name()?)?));
    }
    Ok(Command::Push(ast.to_data_type()?))
}
