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
    CannotSaveToStackIndex(usize),
    InvalidLocalSaveIndex(usize),
    InvalidLocalGetIndex(usize),
    // type errors
    NotAFunction(DataType),
    NotABool(DataType),
    CannotCompareTypes(DataType, DataType),
    InvalidIoAppendTarget(DataType),
    TargetNotAString(DataType),
    CannotConcat(DataType, DataType),
}

#[derive(Debug, Clone)]
pub enum ParserError {
    FileLoadFile(String),
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
    InvalidPart,
    InvalidStringStart,
    InvalidString,
    InvalidStringEscape,
    InvalidAdd,
    InvalidConcat,
    InvalidMul,
    InvalidExp,
}

#[derive(Debug)]
pub enum KscriptError {
    RuntimeError(RuntimeError),
    ParserError(ParserError),
    VmCommandsEmpty,
    CannotParseArgs(String),
    CallDataEmpty,
}
