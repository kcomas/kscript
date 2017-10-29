
extern crate kscript;

use kscript::lang::Kscript;
use kscript::lang::logger::{Logger, VoidLogger, LoggerMode};
use kscript::lang::parser::token::Token;

fn create<T: Logger>(program: &str, logger: T) -> Kscript<T> {
    let mut kscript = Kscript::new(logger);
    kscript.run(program);
    if let Err(kerror) = kscript.run(program) {
        panic!("{:?}", kerror);
    }
    kscript
}

#[test]
fn var_assign_integer() {
    let mut kscript = create("test = 1234", VoidLogger::new(LoggerMode::Void));

    let mabe_token_container = kscript.get_token_container();

    if let Some(ref token_container) = *mabe_token_container {
        let tokens = token_container.get_tokens();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Var("test".to_string()));
        assert_eq!(tokens[1], Token::Assign);
        assert_eq!(tokens[2], Token::Integer(1234));
    } else {
        panic!("Token container not created");
    }
}

#[test]
fn constant_assign_float() {
    let mut kscript = create("TEST = 1234.123", VoidLogger::new(LoggerMode::Void));

    let mabe_token_container = kscript.get_token_container();

    if let Some(ref token_container) = *mabe_token_container {
        let tokens = token_container.get_tokens();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Const("TEST".to_string()));
        assert_eq!(tokens[1], Token::Assign);
        assert_eq!(tokens[2], Token::Float(1234.123));
    } else {
        panic!("Token container not created");
    }
}

#[test]
fn var_assign_math() {}
