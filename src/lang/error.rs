
use std::num::ParseIntError;
use std::num::ParseFloatError;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
    ImpossibleState,
    CheckMismatch(char, usize, usize),
    InvalidVariableChar(char, usize, usize),
    InvalidConstantChar(char, usize, usize),
    IntegerParseFail(ParseIntError),
    FloatParseFail(ParseFloatError),
    FileLoadFail(IoError),
    InvalidArrayOp(char, usize, usize),
    InvaliDictOp(char, usize, usize),
    InvalidConditional(char, usize, usize),
    InvalidIfBlock(char, usize, usize),
    InvalidLoop(char, usize, usize),
    InvalidFunctionArguments(char, usize, usize),
    InvalidRef(char, usize, usize),
    InvalidFunctionBody(char, usize, usize),
    InvalidSystemCommand(char, usize, usize),
    InvalidPass(char, usize, usize),
    InvalidFunctionCall(char, usize, usize),
    InvalidObjectAccess(char, usize, usize),
    InvalidTokenAccess,
    TokenMismatch,
    InvalidRegisterAccess,
}
