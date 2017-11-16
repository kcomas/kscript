
extern crate kscript;

use kscript::lang::Kscript;
use kscript::lang::parser::token::{Token, SystemCommand};
use kscript::lang::logger::{Logger, VoidLogger, LoggerMode};

pub fn create<T: Logger>(program: &str, logger: T) -> Kscript<T> {
    let mut kscript = Kscript::new(logger);
    if let Err(kerror) = kscript.run_build_tokens(program) {
        panic!("{:?}", kerror);
    }
    kscript
}

pub fn get_tokens<T: Logger>(kscript: &Kscript<T>) -> &Vec<Token> {
    let mabe_token_container = kscript.get_token_container();
    let token_container = mabe_token_container.unwrap();
    token_container.get_tokens()
}
