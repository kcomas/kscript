
extern crate kscript;

use kscript::lang::Kscript;
use kscript::lang::logger::{Logger, VoidLogger, LoggerMode};
use kscript::lang::parser::token::Token;

fn create<T: Logger>(program: &str, logger: T) -> Kscript<T> {
    let mut kscript = Kscript::new(logger);
    if let Err(kerror) = kscript.run(program) {
        panic!("{:?}", kerror);
    }
    kscript
}

fn get_tokens<T: Logger>(kscript: &Kscript<T>) -> &Vec<Token> {
    let mabe_token_container = kscript.get_token_container();
    let token_container = mabe_token_container.unwrap();
    token_container.get_tokens()
}

#[test]
fn var_assign_integer() {
    let kscript = create("test = 1234", VoidLogger::new(LoggerMode::Void));

    let tokens = get_tokens(&kscript);
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], Token::Var("test".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(tokens[2], Token::Integer(1234));
}

#[test]
fn constant_assign_float() {
    let kscript = create("TEST = 1234.123", VoidLogger::new(LoggerMode::Void));

    let tokens = get_tokens(&kscript);
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], Token::Const("TEST".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(tokens[2], Token::Float(1234.123));
}

#[test]
fn var_assign_math() {
    let kscript = create(
        "a = (1.234 * ((2 + 4.3) % 2) + 1 ^ 5)",
        VoidLogger::new(LoggerMode::Void),
    );

    let tokens = get_tokens(&kscript);
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], Token::Var("a".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(
        tokens[2],
        Token::Math(vec![
            Token::Float(1.234),
            Token::Multiply,
            Token::Math(vec![
                Token::Math(vec![
                    Token::Integer(2),
                    Token::Add,
                    Token::Float(4.3),
                ]),
                Token::Modulus,
                Token::Integer(2),
            ]),
            Token::Add,
            Token::Integer(1),
            Token::Exponent,
            Token::Integer(5),
        ])
    );
}

#[test]
fn math_io_integer() {
    let kscript = create("(2 * 3) > 1", VoidLogger::new(LoggerMode::Void));

    let tokens = get_tokens(&kscript);
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[1], Token::IoWrite);
    assert_eq!(tokens[2], Token::Integer(1));
}
