use super::ast::Ast;
use super::data_type::DataType;

#[derive(Debug)]
pub enum Error<'a> {
    InvalidComment(&'a str),
    InvalidVar(&'a str),
    InvalidNumber(&'a str),
    InvalidEquals(&'a str),
    InvalidBlock(&'a str),
    InvalidIoWrite(&'a str),
    InvalidArgs(&'a str),
    InvalidAstForCommand(Ast, &'a str),
    FunctionDeclared(String, &'a str),
    FunctionNotDeclared(String, &'a str),
    AstIsNotAFunction(&'a str),
    VarDeclared(String, &'a str),
    VarNotDeclared(String, &'a str),
    AstNotVar(&'a str),
    CannotDeclareSubMain(&'a str),
    CannotConvertToDataType(Ast, &'a str),
    CannotGetIfBody(&'a str),
    MainNotDeclared(&'a str),
    InvalidCommandIndex(usize, &'a str),
    StackEmpty(&'a str),
    InvalidLocal(usize, &'a str),
    CannotCompare(DataType, DataType, &'a str),
    InvalidWriteTarget(DataType, &'a str),
    CannotReturn(&'a str),
}
