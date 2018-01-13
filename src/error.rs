use super::data_type::DataType;
use super::builder::Ast;

#[derive(Debug)]
pub enum RuntimeError {
    // run errors
    StackEmpty,
    CallsEmpty,
    CannotReturn,
    NoMoreCommands,
    InvalidNumberOfArguments,
    ArgumentsNotOnStack(usize, usize),
    CannotLoadArgToStack(usize),
    InvalidLocalSaveIndex(usize),
    InvalidLocalGetIndex(usize),
    // type errors
    NotAFunction(DataType),
    NotABool(DataType),
    CannotCompareTypes(DataType, DataType),
    InvalidIoAppendTarget(DataType),
}

#[derive(Debug, Clone)]
pub enum ParserError {
    InvalidComment,
    InvalidVar,
    InvalidAssign,
    InvalidEquals,
    InvalidNumber,
    InvalidFloat,
    InvalidIoWrite,
    InvalidIoAppend,
    InvalidBlockStart,
    InvalidBlock,
    InvalidItem,
    InvalidTotalArgs(usize, usize),
    CannotConvetAstToDataType(Ast),
    CannotConvertAstToCommand(Ast),
    CannotSaveFromAst(Ast),
}
