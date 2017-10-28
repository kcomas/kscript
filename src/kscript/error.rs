
#[derive(Debug)]
pub enum Error {
    CheckMismatch(char, usize, usize),
    InvalidVariableChar(char, usize, usize),
    InvalidConstantChar(char, usize, usize),
}
