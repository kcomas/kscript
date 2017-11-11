
extern crate kscript;

use kscript::lang::Kscript;
use kscript::lang::logger::{Logger, VoidLogger, LoggerMode};
use kscript::lang::parser::token::{Token, SystemCommand};

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
                    Token::Addition,
                    Token::Float(4.3),
                ]),
                Token::Modulus,
                Token::Integer(2),
            ]),
            Token::Addition,
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
    assert_eq!(
        tokens[0],
        Token::Math(vec![Token::Integer(2), Token::Multiply, Token::Integer(3)])
    );
    assert_eq!(tokens[1], Token::IoWrite);
    assert_eq!(tokens[2], Token::Integer(1));
}

#[test]
fn comment_op_comment() {
    let kscript = create(
        "# this is a comment\n a = 1 # another comment",
        VoidLogger::new(LoggerMode::Void),
    );

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0], Token::Comment(" this is a comment".to_string()));
    assert_eq!(tokens[1], Token::Var("a".to_string()));
    assert_eq!(tokens[2], Token::Assign);
    assert_eq!(tokens[3], Token::Integer(1));
    assert_eq!(tokens[4], Token::Comment(" another comment".to_string()));
}

#[test]
fn var_assign_file() {
    let kscript = create("myfile = 'hello'", VoidLogger::new(LoggerMode::Void));

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], Token::Var("myfile".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(tokens[2], Token::File("hello".to_string()));
}

#[test]
fn var_assign_string() {
    let kscript = create("mystr = \"test # str\"", VoidLogger::new(LoggerMode::Void));

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], Token::Var("mystr".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(tokens[2], Token::String("test # str".to_string()));
}

#[test]
fn var_assign_array() {
    let kscript = create(
        "a = @[1, @[1.34, \"herp\"], (1 + 2 * 3), 1234]",
        VoidLogger::new(LoggerMode::Void),
    );

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], Token::Var("a".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(
        tokens[2],
        Token::Array(vec![
            Token::Integer(1),
            Token::Array(vec![
                Token::Float(1.34),
                Token::String("herp".to_string()),
            ]),
            Token::Math(vec![
                Token::Integer(1),
                Token::Addition,
                Token::Integer(2),
                Token::Multiply,
                Token::Integer(3),
            ]),
            Token::Integer(1234),
        ])
    );
}

#[test]
fn var_assign_dict() {
    let kscript = create(
        "d = %[\"asdf\": 1234, \"sub\": %[\"merp\": 3.45], \"arr\": @[1, 2, 4], \"herp\": \"derp\"]",
        VoidLogger::new(LoggerMode::Void),
    );

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], Token::Var("d".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(
        tokens[2],
        Token::Dict(
            vec![
                Token::String("asdf".to_string()),
                Token::String("sub".to_string()),
                Token::String("arr".to_string()),
                Token::String("herp".to_string()),
            ],
            vec![
                Token::Integer(1234),
                Token::Dict(
                    vec![Token::String("merp".to_string())],
                    vec![Token::Float(3.45)]
                ),
                Token::Array(vec![
                    Token::Integer(1),
                    Token::Integer(2),
                    Token::Integer(4),
                ]),
                Token::String("derp".to_string()),
            ],
        )
    );
}

#[test]
fn var_assign_bool_const_assign_bool() {
    let kscript = create("test = t; TESTD = f", VoidLogger::new(LoggerMode::Void));

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 7);
    assert_eq!(tokens[0], Token::Var("test".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(tokens[2], Token::Bool(true));
    assert_eq!(tokens[3], Token::End);
    assert_eq!(tokens[4], Token::Const("TESTD".to_string()));
    assert_eq!(tokens[5], Token::Assign);
    assert_eq!(tokens[6], Token::Bool(false));
}

#[test]
fn vars_const_with_numbers() {
    let kscript = create(
        "py3 = 3; 23a = 3.12; 1S3 = 4",
        VoidLogger::new(LoggerMode::Void),
    );

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 11);
    assert_eq!(tokens[0], Token::Var("py3".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(tokens[2], Token::Integer(3));
    assert_eq!(tokens[3], Token::End);
    assert_eq!(tokens[4], Token::Var("23a".to_string()));
    assert_eq!(tokens[5], Token::Assign);
    assert_eq!(tokens[6], Token::Float(3.12));
    assert_eq!(tokens[7], Token::End);
    assert_eq!(tokens[8], Token::Const("1S3".to_string()));
    assert_eq!(tokens[9], Token::Assign);
    assert_eq!(tokens[10], Token::Integer(4));
}

#[test]
fn assign_conditional_true_false() {
    let kscript = create(
        "a = ?1 == 2{a = 3}{b = \"test\"}",
        VoidLogger::new(LoggerMode::Void),
    );

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], Token::Var("a".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(
        tokens[2],
        Token::If(
            Box::new(Token::Conditional(
                Box::new(Token::Integer(1)),
                Box::new(Token::Equals),
                Box::new(Token::Integer(2)),
            )),
            vec![
                Token::Var("a".to_string()),
                Token::Assign,
                Token::Integer(3),
            ],
            vec![
                Token::Var("b".to_string()),
                Token::Assign,
                Token::String("test".to_string()),
            ],
        )
    );
}

#[test]
fn assign_loop_print() {
    let kscript = create(
        "a = 1; $a<5{a = (a + 1)} a > 1",
        VoidLogger::new(LoggerMode::Void),
    );

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 8);
    assert_eq!(tokens[0], Token::Var("a".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(tokens[2], Token::Integer(1));
    assert_eq!(tokens[3], Token::End);
    assert_eq!(
        tokens[4],
        Token::Loop(
            Box::new(Token::Conditional(
                Box::new(Token::Var("a".to_string())),
                Box::new(Token::Less),
                Box::new(Token::Integer(5)),
            )),
            vec![
                Token::Var("a".to_string()),
                Token::Assign,
                Token::Math(vec![
                    Token::Var("a".to_string()),
                    Token::Addition,
                    Token::Integer(1),
                ]),
            ],
        )
    );
    assert_eq!(tokens[5], Token::Var("a".to_string()));
    assert_eq!(tokens[6], Token::IoWrite);
    assert_eq!(tokens[7], Token::Integer(1));
}

#[test]
fn var_assign_var_function() {
    let kscript = create(
        "a = 1; b = { (a, &e, c) e = c; d }",
        VoidLogger::new(LoggerMode::Void),
    );

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 7);
    assert_eq!(tokens[0], Token::Var("a".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(tokens[2], Token::Integer(1));
    assert_eq!(tokens[3], Token::End);
    assert_eq!(tokens[4], Token::Var("b".to_string()));
    assert_eq!(tokens[5], Token::Assign);
    assert_eq!(
        tokens[6],
        Token::Function(
            vec![
                Token::Var("a".to_string()),
                Token::Ref(Box::new(Token::Var("e".to_string()))),
                Token::Var("c".to_string()),
            ],
            vec![
                Token::Var("e".to_string()),
                Token::Assign,
                Token::Var("c".to_string()),
                Token::End,
                Token::Var("d".to_string()),
            ],
        )
    )
}

#[test]
fn assign_system_command() {
    let kscript = create("a = 1; \\\\1; b = 2", VoidLogger::new(LoggerMode::Void));

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 9);
    assert_eq!(tokens[0], Token::Var("a".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(tokens[2], Token::Integer(1));
    assert_eq!(tokens[3], Token::End);
    assert_eq!(tokens[4], Token::System(SystemCommand::Exit(1)));
    assert_eq!(tokens[5], Token::End);
    assert_eq!(tokens[6], Token::Var("b".to_string()));
    assert_eq!(tokens[7], Token::Assign);
    assert_eq!(tokens[8], Token::Integer(2));
}
