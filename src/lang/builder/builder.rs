use std::rc::Rc;
use super::super::command::Command;
use super::super::data_type::DataType;
use super::super::error::ParserError;
use super::ast::{Ast, AstArgs};

pub fn load_commands_from_ast(ast: &Vec<Ast>) -> Result<Vec<Command>, ParserError> {
    let mut new_commands = Vec::new();
    let mut current_index = 0;
    if ast.len() == 1 {
        if let Some(ref args) = ast[0].is_function_call() {
            let mut call_commands = build_function_call(args)?;
            new_commands.append(&mut call_commands);
            new_commands.push(Command::CallSelf);
            return Ok(new_commands);
        } else {
            return Ok(vec![ast_to_command(&ast[0])?]);
        }
    }
    while current_index < ast.len() {
        let mut total_look_back = ast[current_index].num_look_back();
        if total_look_back > 0 {
            can_look_back(current_index, total_look_back)?;
            if ast[current_index].is_assign() {
                // build n - 1
                new_commands.push(ast_to_command(&ast[current_index - 1])?);
                // save to n - 2
                let save_cmd = match ast[current_index - 2] {
                    Ast::VarLocal(_, id) => Command::SaveLocal(id),
                    _ => {
                        return Err(ParserError::CannotSaveFromAst(
                            ast[current_index - 2].clone(),
                        ))
                    }
                };
                new_commands.push(save_cmd);
            } else {
                while total_look_back > 0 {
                    let current_look_back_index = current_index - total_look_back;
                    if let Some(ref args) = ast[current_look_back_index].is_function_call() {
                        let mut call_commands = build_function_call(args)?;
                        new_commands.append(&mut call_commands);
                        // take a look at n - 1
                        if current_look_back_index > 0
                            && ast[current_look_back_index - 1].can_call()
                        {
                            new_commands.push(ast_to_command(&ast[current_look_back_index - 1])?);
                            new_commands.push(Command::Call);
                        } else {
                            new_commands.push(Command::CallSelf);
                        }
                    } else {
                        new_commands.push(ast_to_command(&ast[current_look_back_index])?);
                    }
                    total_look_back -= 1;
                }
                new_commands.push(ast_to_command(&ast[current_index])?);
            }
        } else if let Some(if_body) = ast[current_index].is_if() {
            let mut total_if_commands = Vec::new();
            for if_ast in if_body.iter() {
                let mut if_commands = load_commands_from_ast(if_ast)?;
                total_if_commands.append(&mut if_commands);
            }
            // add jump command
            new_commands.push(Command::JumpIfFalse(total_if_commands.len() + 1));
            new_commands.append(&mut total_if_commands);
        }
        current_index += 1;
    }
    Ok(new_commands)
}

fn can_look_back(mut current_index: usize, mut total_look_back: usize) -> Result<(), ParserError> {
    while total_look_back > 0 {
        if current_index == 0 {
            return Err(ParserError::InvalidTotalArgs(
                current_index,
                total_look_back,
            ));
        }
        total_look_back -= 1;
        current_index -= 1;
    }
    Ok(())
}

fn ast_to_command(ast: &Ast) -> Result<Command, ParserError> {
    if ast.is_data() {
        return Ok(Command::PushStack(ast_to_data_type(ast)?));
    }
    let cmd = match *ast {
        Ast::VarArg(_, id) => Command::LoadStackArg(id),
        Ast::VarLocal(_, id) => Command::LoadLocal(id),
        Ast::Return => Command::Return,
        Ast::Equals => Command::Equals,
        Ast::Add => Command::Add,
        Ast::Sub => Command::Sub,
        Ast::IoAppend => Command::IoAppend,
        _ => return Err(ParserError::CannotConvertAstToCommand(ast.clone())),
    };
    Ok(cmd)
}

fn ast_to_data_type(ast: &Ast) -> Result<DataType, ParserError> {
    let dt = match *ast {
        Ast::Bool(b) => DataType::Bool(b),
        Ast::Integer(int) => DataType::Integer(int),
        Ast::Float(float) => DataType::Float(float),
        Ast::Function(ref args, ref body) => {
            let num_args = args.len();
            let mut function_commands = Vec::new();
            for body_part in body.iter() {
                let mut sub_commands = load_commands_from_ast(body_part)?;
                function_commands.append(&mut sub_commands);
            }
            let mut add_return = false;
            if let Some(cmd) = function_commands.last() {
                if !cmd.is_return() {
                    add_return = true;
                }
            }
            if add_return {
                function_commands.push(Command::Return);
            }
            DataType::Function(Rc::new(function_commands), num_args)
        }
        _ => return Err(ParserError::CannotConvetAstToDataType(ast.clone())),
    };
    Ok(dt)
}

pub fn build_function_call(args: &AstArgs) -> Result<Vec<Command>, ParserError> {
    let mut call_commands = Vec::new();
    for arg in args.iter() {
        for arg_group in arg.iter() {
            let mut arg_commands = load_commands_from_ast(arg_group)?;
            call_commands.append(&mut arg_commands);
        }
    }
    Ok(call_commands)
}
