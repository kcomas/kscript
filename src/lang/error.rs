
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
}
