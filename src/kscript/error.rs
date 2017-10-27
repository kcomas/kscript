
#[derive(Debug)]
pub enum Error {
    // wrong char, char pos, line
    InvalidEndChar(char, usize, usize),
}
