use std::rc::Rc;
use std::cell::RefCell;
use super::data_type::SharedDataType;
use super::ast::Ast;
use super::error::Error;
use super::symbol::SymbolTable;

#[derive(Debug)]
pub enum Command {
    AddLocals,
    RemoveLocals,
    // add to stack
    Push(SharedDataType),
    // remove from stack
    // Pop,
    // load argument from the locals stack
    Load(usize),
    // save value to locals stack
    Save(usize),
    // create an array with values from stack
    MakeArray(usize),
    Equals,
    Add,
    Sub,
    Mul,
    Exp,
    Div,
    Rem,
    IoWrite,
    IoAppend,
    // jump to position if false
    Jmpf(usize),
    // number of args, function position
    Call(usize, usize),
    Return,
    // exit code
    Halt(i32),
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

#[derive(Debug)]
pub struct CommandState {
    num_args: usize,
    added_locals: bool,
}

impl CommandState {
    pub fn new(num_args: usize) -> CommandState {
        CommandState {
            num_args: num_args,
            added_locals: false,
        }
    }

    pub fn get_num_args(&self) -> usize {
        self.num_args
    }

    pub fn get_added_locals(&self) -> bool {
        self.added_locals
    }

    pub fn added_locals(&mut self) {
        self.added_locals = true;
    }
}

pub fn load_commands<'a>(
    ast: &mut Vec<Ast>,
    commands: &mut Vec<Command>,
    symbols: &mut SymbolTable,
    command_state: &mut CommandState,
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
                    let mut sub_command_state;
                    {
                        let args = ast[highest_presedence_index].get_function_args()?;
                        // convert the args to indexes
                        for arg in args.iter() {
                            if let Some(var) = arg.get(0) {
                                let var_name = var.get_var_name()?;
                                function_symbol_table.register_var(var_name)?;
                            }
                        }
                        sub_command_state = CommandState::new(args.len());
                    }
                    let fn_body = ast[highest_presedence_index].get_function_body_mut()?;
                    load_commands(
                        fn_body,
                        commands,
                        &mut function_symbol_table,
                        &mut sub_command_state,
                    )?;
                    if sub_command_state.get_added_locals() {
                        commands.push(Command::RemoveLocals);
                    }
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
                    build_function_call(
                        ast,
                        highest_presedence_index,
                        commands,
                        symbols,
                        command_state,
                    )?;
                }
            } else if ast[highest_presedence_index].is_if() {
                let mut if_commands = Vec::new();
                load_commands(
                    ast[highest_presedence_index].get_if_body_mut()?,
                    &mut if_commands,
                    symbols,
                    command_state,
                )?;
                let jmpf = Command::Jmpf(commands.len() + if_commands.len() + 1);
                commands.push(jmpf);
                commands.append(&mut if_commands);
            } else if ast[highest_presedence_index].is_group() {
                load_commands(
                    ast[highest_presedence_index].get_group_body_mut()?,
                    commands,
                    symbols,
                    command_state,
                )?;
            } else if ast[highest_presedence_index].is_array() {
                build_array(
                    ast,
                    highest_presedence_index,
                    commands,
                    symbols,
                    command_state,
                )?;
            } else {
                add_commands(
                    ast,
                    highest_presedence_index,
                    commands,
                    symbols,
                    command_state,
                )?;
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
    command_state: &mut CommandState,
) -> Result<(), Error<'a>> {
    let fn_index = symbols.get_function_index(ast[index].get_function_name()?)?;
    let args = ast[index].get_function_args_mut()?;
    for arg in args.iter_mut() {
        load_commands(arg, commands, symbols, command_state)?;
    }
    commands.push(Command::Call(args.len(), fn_index));
    Ok(())
}

fn build_array<'a>(
    ast: &mut Vec<Ast>,
    index: usize,
    commands: &mut Vec<Command>,
    symbols: &mut SymbolTable,
    command_state: &mut CommandState,
) -> Result<(), Error<'a>> {
    let items = ast[index].get_array_body_mut()?;
    for item in items.iter_mut() {
        load_commands(item, commands, symbols, command_state)?;
    }
    commands.push(Command::MakeArray(items.len()));
    Ok(())
}

fn add_commands<'a>(
    ast: &mut Vec<Ast>,
    index: usize,
    commands: &mut Vec<Command>,
    symbols: &mut SymbolTable,
    command_state: &mut CommandState,
) -> Result<(), Error<'a>> {
    if ast[index].is_data() {
        transform_command(&ast[index], commands, symbols, command_state)?;
        return Ok(());
    }
    if ast[index].is_dyadic() {
        if ast[index].is_assign() {
            // get the right
            if index + 1 >= ast.len() {
                return Err(Error::CannotAssign("Right side does not exist"));
            }
            build_command(ast, index + 1, commands, symbols, command_state)?;
            if index == 0 {
                return Err(Error::CannotAssign("Left side does not exist"));
            }
            let var_index = symbols.get_var_index(ast[index - 1].get_var_name()?)?;
            check_locals(var_index, commands, command_state);
            commands.push(Command::Save(var_index));
            ast[index - 1] = Ast::Used;
            return Ok(());
        } else {
            if index > 0 {
                build_command(ast, index - 1, commands, symbols, command_state)?;
            }
            if index < ast.len() {
                build_command(ast, index + 1, commands, symbols, command_state)?;
            }
        }
    }
    if ast[index].is_monadic_left() {
        if index > 0 {
            build_command(ast, index - 1, commands, symbols, command_state)?;
        }
    }
    let command = match ast[index] {
        Ast::Equals => Command::Equals,
        Ast::Add => Command::Add,
        Ast::Sub => Command::Sub,
        Ast::Mul => Command::Mul,
        Ast::Exp => Command::Exp,
        Ast::Div => Command::Div,
        Ast::Rem => Command::Rem,
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
    command_state: &mut CommandState,
) -> Result<(), Error<'a>> {
    if !ast[next_index].is_used() {
        if ast[next_index].is_function() {
            build_function_call(ast, next_index, commands, symbols, command_state)?;
        } else if ast[next_index].is_group() {
            load_commands(
                ast[next_index].get_group_body_mut()?,
                commands,
                symbols,
                command_state,
            )?;
        } else if ast[next_index].is_array() {
            build_array(ast, next_index, commands, symbols, command_state)?;
        } else {
            transform_command(&ast[next_index], commands, symbols, command_state)?;
        }
        ast[next_index] = Ast::Used;
    }
    Ok(())
}

fn transform_command<'a>(
    ast: &Ast,
    commands: &mut Vec<Command>,
    symbols: &mut SymbolTable,
    command_state: &mut CommandState,
) -> Result<(), Error<'a>> {
    if ast.is_var() {
        let var_index = symbols.get_var_index(ast.get_var_name()?)?;
        check_locals(var_index, commands, command_state);
        commands.push(Command::Load(var_index));
    } else if ast.is_data() {
        commands.push(Command::Push(Rc::new(RefCell::new(ast.to_data_type()?))));
    } else {
        return Err(Error::UnknownAstType(
            ast.clone(),
            "Not known what to do with this ast",
        ));
    }
    Ok(())
}

fn check_locals(index: usize, commands: &mut Vec<Command>, command_state: &mut CommandState) {
    if index >= command_state.get_num_args() && !command_state.get_added_locals() {
        commands.push(Command::AddLocals);
        command_state.added_locals();
    }
}
