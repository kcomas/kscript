mod command;

use std::rc::Rc;
use std::cell::RefCell;
use super::ast::Ast;
use super::error::Error;
use super::symbol::SymbolTable;
pub use self::command::Command;

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
            set_used(&mut ast[highest_presedence_index])?;
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
            build_command(
                ast,
                index + 1,
                commands,
                symbols,
                command_state,
                MoveEndDirection::Right,
            )?;
            if index == 0 {
                return Err(Error::CannotAssign("Left side does not exist"));
            }
            let mut find_index = index - 1;
            let mut is_access = false;
            loop {
                if ast[find_index].is_var() {
                    break;
                }
                if ast[find_index].is_used_var() {
                    is_access = true;
                    break;
                }
                if find_index == 0 {
                    return Err(Error::CannotAssign("Left side does not exist after search"));
                }
                find_index -= 1;
            }
            if !is_access {
                let var_index = symbols.get_var_index(ast[find_index].get_var_name()?)?;
                let (is_local, save_index) = check_locals(var_index, commands, command_state);
                if is_local {
                    commands.push(Command::SaveLocal(save_index));
                } else {
                    commands.push(Command::SaveStack(save_index));
                }
            } else {
                commands.push(Command::SaveAccess);
            }
            set_used(&mut ast[find_index])?;
            return Ok(());
        } else {
            if index > 0 {
                build_command(
                    ast,
                    index - 1,
                    commands,
                    symbols,
                    command_state,
                    MoveEndDirection::Left,
                )?;
            }
            if index < ast.len() {
                build_command(
                    ast,
                    index + 1,
                    commands,
                    symbols,
                    command_state,
                    MoveEndDirection::Right,
                )?;
            }
        }
    }
    if ast[index].is_monadic_left() {
        if index > 0 {
            build_command(
                ast,
                index - 1,
                commands,
                symbols,
                command_state,
                MoveEndDirection::Left,
            )?;
        }
    }
    let command;
    if ast[index].is_access() {
        load_commands(
            ast[index].get_access_body_mut()?,
            commands,
            symbols,
            command_state,
        )?;
        command = Command::Access;
    } else {
        command = match ast[index] {
            Ast::ArrayPush => Command::ArrayPush,
            Ast::ArrayPop => Command::ArrayPop,
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
    }
    commands.push(command);
    Ok(())
}

enum MoveEndDirection {
    Left,
    Right,
}

impl MoveEndDirection {
    pub fn is_left(&self) -> bool {
        if let MoveEndDirection::Left = *self {
            return true;
        }
        false
    }

    pub fn is_right(&self) -> bool {
        if let MoveEndDirection::Right = *self {
            return true;
        }
        false
    }
}

fn build_command<'a>(
    ast: &mut Vec<Ast>,
    next_index: usize,
    commands: &mut Vec<Command>,
    symbols: &mut SymbolTable,
    command_state: &mut CommandState,
    move_direction: MoveEndDirection,
) -> Result<(), Error<'a>> {
    if ast[next_index].is_end() {
        // move to next node
        if move_direction.is_left() && next_index > 0 {
            return build_command(
                ast,
                next_index - 1,
                commands,
                symbols,
                command_state,
                move_direction,
            );
        } else if move_direction.is_right() && next_index < ast.len() {
            return build_command(
                ast,
                next_index + 1,
                commands,
                symbols,
                command_state,
                move_direction,
            );
        }
        return Err(Error::CannotMoveToBuildType(
            next_index,
            "Cannot move to find next ast",
        ));
    }
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
        set_used(&mut ast[next_index])?;
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
        let (is_local, load_index) = check_locals(var_index, commands, command_state);
        if is_local {
            commands.push(Command::LoadLocal(load_index));
        } else {
            commands.push(Command::LoadStack(load_index));
        }
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

fn check_locals(
    index: usize,
    commands: &mut Vec<Command>,
    command_state: &mut CommandState,
) -> (bool, usize) {
    // return the new index and if it is a local
    if index >= command_state.get_num_args() && !command_state.get_added_locals() {
        commands.push(Command::AddLocals);
        command_state.added_locals();
    }
    // check if it is a local
    if index < command_state.get_num_args() {
        return (false, index);
    }
    (true, index - command_state.get_num_args())
}

fn set_used<'a>(ast: &mut Ast) -> Result<(), Error<'a>> {
    if ast.is_var() {
        *ast = Ast::UsedVar(ast.get_var_name()?.to_string());
    } else {
        *ast = Ast::Used;
    }
    Ok(())
}
