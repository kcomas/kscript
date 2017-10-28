
extern crate kscript;

use kscript::lang::Kscript;
use kscript::lang::logger::{Logger, VoidLogger, LoggerMode};
use kscript::lang::parser::token::Token;

#[test]
fn var_assign_integer() {
    let mut kscript = Kscript::new(VoidLogger::new(LoggerMode::Void));
    if let Err(kerror) = kscript.run("test = 1234") {
        panic!("{:?}", kerror);
    }

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
