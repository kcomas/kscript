use super::ast::{Ast, AstBody};
use super::command::Command;
use super::memory::{Function, Memory, MemoryAddress};
use super::data::DataHolder;
use super::error::BuilderError;

pub fn build_commands(
    ast: &AstBody,
    num_arguments: usize,
    memory: &mut Memory,
) -> Result<MemoryAddress, BuilderError> {
    let mut commands = Vec::new();
    for ast_section in ast.iter() {
        for ast_item in ast_section.iter() {
            if ast_item.has_body() {

            } else {
                commands.push(ast_to_command(ast_item, memory)?);
            }
        }
    }

    let address = memory.insert(
        DataHolder::Function(Function::new(commands, num_arguments)),
        false,
    );
    Ok(address)
}

// fn ast_to_commands(ast_with_body: &Ast, memory: &mut Memory) -> Result<Vec<Command, BuilderError> {}

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
