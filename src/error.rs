#[derive(Debug)]
pub enum Error<'a> {
    InvalidComment(&'a str),
    InvalidVar(&'a str),
    InvalidNumber(&'a str),
    InvalidEquals(&'a str),
    InvalidBlock(&'a str),
    InvalidIoWrite(&'a str),
    InvalidArgs(&'a str),
    FunctionDeclared(String, &'a str),
}
