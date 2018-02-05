use super::ast::{Ast, AstBody};
use super::command::Command;
use super::memory::{Function, Memory, MemoryAddress};
use super::data::DataHolder;
use super::error::BuilderError;

pub fn build_commands(
    ast: &AstBody,
    num_arguments: usize,
    memory: &mut Memory,
    is_entry: bool,
) -> Result<MemoryAddress, BuilderError> {
    let mut commands = Vec::new();
    build_body(ast, memory, &mut commands)?;

    let mut add_halt = false;
    let mut add_return = false;
    if is_entry {
        add_halt = match commands.last() {
            Some(cmd) => !cmd.is_halt(),
            None => true,
        };
    } else {
        add_return = match commands.last() {
            Some(cmd) => !cmd.is_return(),
            None => true,
        };
    }

    if add_halt {
        commands.push(Command::Halt(0));
    } else if add_return {
        commands.push(Command::Return);
    }

    let address = memory.insert(
        DataHolder::Function(Function::new(commands, num_arguments)),
        false,
    );
    Ok(address)
}

fn build_body(
    ast: &AstBody,
    memory: &mut Memory,
    commands: &mut Vec<Command>,
) -> Result<(), BuilderError> {
    for ast_section in ast.iter() {
        for ast_item in ast_section.iter() {
            if ast_item.has_body() {
                commands.append(&mut ast_to_commands(ast_item, memory)?);
            } else {
                commands.push(ast_to_command(ast_item, memory)?);
            }
        }
    }
    Ok(())
}

fn ast_to_commands(ast_with_body: &Ast, memory: &mut Memory) -> Result<Vec<Command>, BuilderError> {
    let mut new_commands = Vec::new();
    match *ast_with_body {
        Ast::Function(num_arguments, ref body) => {
            let function_memory = build_commands(body, num_arguments, memory, false)?;
            new_commands.push(Command::PushStack(function_memory));
        }
        Ast::LocalFunctionCall(local, ref arg_body) => {
            build_body(arg_body, memory, &mut new_commands)?;
            new_commands.push(Command::LoadLocal(local));
            new_commands.push(Command::Call);
        }
        Ast::ArgFunctionCall(arg, ref arg_body) => {
            build_body(arg_body, memory, &mut new_commands)?;
            new_commands.push(Command::LoadArgument(arg));
            new_commands.push(Command::Call);
        }
        Ast::SelfFuctionCall(ref arg_body) => {
            build_body(arg_body, memory, &mut new_commands)?;
            new_commands.push(Command::CallSelf);
        }
        Ast::IfStatement(ref if_body) => {
            let mut if_commands = Vec::new();
            build_body(if_body, memory, &mut if_commands)?;
            let if_length = if_commands.len();
            new_commands.push(Command::JumpIfFalse(if_length));
            new_commands.append(&mut if_commands);
        }
        _ => return Err(BuilderError::InvalidAstWithBody),
    };
    Ok(new_commands)
}

fn ast_to_command(ast_item: &Ast, memory: &mut Memory) -> Result<Command, BuilderError> {
    let cmd = match *ast_item {
        Ast::Integer(int) => Command::PushStack(memory.insert(DataHolder::Integer(int), true)),
        Ast::Float(float) => Command::PushStack(memory.insert(DataHolder::Float(float), true)),
        Ast::VarLocal(index) => Command::LoadLocal(index),
        Ast::SaveLocal(index) => Command::SaveLocal(index),
        Ast::VarArg(index) => Command::LoadArgument(index),
        Ast::SaveArg(index) => Command::SaveArgument(index),
        Ast::Add => Command::Add,
        Ast::Sub => Command::Sub,
        Ast::Return => Command::Return,
        Ast::Equals => Command::Equals,
        Ast::IoWrite => Command::IoWrite,
        Ast::IoAppend => Command::IoAppend,
        _ => return Err(BuilderError::InvalidSingleAst),
    };
    Ok(cmd)
}
