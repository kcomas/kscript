
extern crate kscript;

use kscript::lang::Kscript;
use kscript::lang::logger::{Logger, VoidLogger, LoggerMode};
use kscript::lang::parser::token::{Token, SystemCommand};

fn create_parser<T: Logger>(program: &str, logger: T) -> Kscript<T> {
    let mut kscript = Kscript::new(logger);
    kscript.run_build_tokens(program).unwrap();
    kscript
}


fn get_tokens<T: Logger>(kscript: &Kscript<T>) -> &Vec<Token> {
    kscript.get_tokens()
}

fn last_is_end(tokens: &Vec<Token>) {
    assert_eq!(*tokens.last().unwrap(), Token::End);
}

#[test]
fn var_assign_integer() {
    let kscript = create_parser("test = 1234", VoidLogger::new(LoggerMode::Void));

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0], Token::Var("test".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(tokens[2], Token::Integer(1234));
    last_is_end(&tokens);
}

#[test]
fn constant_assign_float() {
    let kscript = create_parser("TEST = 1234.123", VoidLogger::new(LoggerMode::Void));

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0], Token::Const("TEST".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(tokens[2], Token::Float(1234.123));
    last_is_end(&tokens);
}

#[test]
fn var_assign_math() {
    let kscript = create_parser(
        "a = (1.234 * ((2 + 4.3) % 2) + 1 ^ 5)",
        VoidLogger::new(LoggerMode::Void),
    );

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 4);
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
    last_is_end(&tokens);
}

#[test]
fn math_io_integer() {
    let kscript = create_parser("(2 * 3) > 1", VoidLogger::new(LoggerMode::Void));

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 4);
    assert_eq!(
        tokens[0],
        Token::Math(vec![Token::Integer(2), Token::Multiply, Token::Integer(3)])
    );
    assert_eq!(tokens[1], Token::IoWrite);
    assert_eq!(tokens[2], Token::Integer(1));
    last_is_end(&tokens);
}

#[test]
fn math_from_access() {
    let kscript = create_parser(
        "a = @[2, 5]; b = {|| 4}; c = %[\"t\": 2, \"g\": 1]; d = (a[1] + b|| + c[\"g\"]); d >> 1",
        VoidLogger::new(LoggerMode::Void),
    );

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 20);
    assert_eq!(tokens[0], Token::Var("a".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(
        tokens[2],
        Token::Array(vec![Token::Integer(2), Token::Integer(5)])
    );
    assert_eq!(tokens[3], Token::End);
    assert_eq!(tokens[4], Token::Var("b".to_string()));
    assert_eq!(tokens[5], Token::Assign);
    assert_eq!(tokens[6], Token::Function(vec![], vec![Token::Integer(4)]));
    assert_eq!(tokens[7], Token::End);
    assert_eq!(tokens[8], Token::Var("c".to_string()));
    assert_eq!(tokens[9], Token::Assign);
    assert_eq!(
        tokens[10],
        Token::Dict(
            vec![
                Token::String("t".to_string()),
                Token::String("g".to_string()),
            ],
            vec![Token::Integer(2), Token::Integer(1)],
        )
    );
    assert_eq!(tokens[11], Token::End);
    assert_eq!(tokens[12], Token::Var("d".to_string()));
    assert_eq!(tokens[13], Token::Assign);
    assert_eq!(
        tokens[14],
        Token::Math(vec![
            Token::ObjectAccess(
                Box::new(Token::Var("a".to_string())),
                Box::new(Token::Integer(1))
            ),
            Token::Addition,
            Token::FunctionCall(
                Box::new(Token::Var("b".to_string())),
                vec![]
            ),
            Token::Addition,
            Token::ObjectAccess(
                Box::new(Token::Var("c".to_string())),
                Box::new(Token::String("g".to_string()))
            ),
        ])
    );
    assert_eq!(tokens[15], Token::End);
    assert_eq!(tokens[16], Token::Var("d".to_string()));
    assert_eq!(tokens[17], Token::IoAppend);
    assert_eq!(tokens[18], Token::Integer(1));
    last_is_end(&tokens);
}

#[test]
fn comment_op_comment() {
    let kscript = create_parser(
        "# this is a comment\n a = 1 # another comment",
        VoidLogger::new(LoggerMode::Void),
    );

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0], Token::Comment(" this is a comment".to_string()));
    assert_eq!(tokens[1], Token::Var("a".to_string()));
    assert_eq!(tokens[2], Token::Assign);
    assert_eq!(tokens[3], Token::Integer(1));
    assert_eq!(tokens[4], Token::Comment(" another comment".to_string()));
    last_is_end(&tokens);
}

#[test]
fn var_assign_file() {
    let kscript = create_parser("myfile = 'hello'", VoidLogger::new(LoggerMode::Void));

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0], Token::Var("myfile".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(tokens[2], Token::File("hello".to_string()));
    last_is_end(&tokens);
}

#[test]
fn var_assign_string() {
    let kscript = create_parser("mystr = \"test # str\"", VoidLogger::new(LoggerMode::Void));

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0], Token::Var("mystr".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(tokens[2], Token::String("test # str".to_string()));
    last_is_end(&tokens);
}

#[test]
fn var_assign_array() {
    let kscript = create_parser(
        "a = @[1, @[1.34, \"herp\"], (1 + 2 * 3), 1234]",
        VoidLogger::new(LoggerMode::Void),
    );

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 4);
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
    last_is_end(&tokens);
}

#[test]
fn var_assign_dict() {
    let kscript = create_parser(
        "d = %[\"asdf\": 1234, \"sub\": %[\"merp\": 3.45], \"arr\": @[1, 2, 4], \"herp\": \"derp\"]",
        VoidLogger::new(LoggerMode::Void),
    );

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 4);
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
    last_is_end(&tokens);
}

#[test]
fn var_assign_bool_const_assign_bool() {
    let kscript = create_parser("test = t; TESTD = f", VoidLogger::new(LoggerMode::Void));

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 8);
    assert_eq!(tokens[0], Token::Var("test".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(tokens[2], Token::Bool(true));
    assert_eq!(tokens[3], Token::End);
    assert_eq!(tokens[4], Token::Const("TESTD".to_string()));
    assert_eq!(tokens[5], Token::Assign);
    assert_eq!(tokens[6], Token::Bool(false));
    last_is_end(&tokens);
}

#[test]
fn vars_const_with_numbers() {
    let kscript = create_parser(
        "py3 = 3; 23a = 3.12; 1S3 = 4",
        VoidLogger::new(LoggerMode::Void),
    );

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 12);
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
    last_is_end(&tokens);
}

#[test]
fn assign_conditional_true_false() {
    let kscript = create_parser(
        "?1 == 2{a = 3}{b = \"test\"}",
        VoidLogger::new(LoggerMode::Void),
    );

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 2);
    assert_eq!(
        tokens[0],
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
    last_is_end(&tokens);
}

#[test]
fn nested_conditionial() {
    let kscript = create_parser("a = ? ?1==1^? 3== 2", VoidLogger::new(LoggerMode::Void));

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0], Token::Var("a".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(
        tokens[2],
        Token::Conditional(
            Box::new(Token::Conditional(
                Box::new(Token::Integer(1)),
                Box::new(Token::Equals),
                Box::new(Token::Integer(1)),
            )),
            Box::new(Token::Or),
            Box::new(Token::Conditional(
                Box::new(Token::Integer(3)),
                Box::new(Token::Equals),
                Box::new(Token::Integer(2)),
            )),
        )
    );
    last_is_end(&tokens);
}

#[test]
fn nested_conditionals_with_nested_data() {
    let kscript = create_parser(
        "c=@[@[2]];a=??1=={|a|a}|1|&?2==c[0][0]",
        VoidLogger::new(LoggerMode::Void),
    );

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 8);
    assert_eq!(tokens[0], Token::Var("c".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(
        tokens[2],
        Token::Array(vec![Token::Array(vec![Token::Integer(2)])])
    );
    assert_eq!(tokens[3], Token::End);
    assert_eq!(tokens[4], Token::Var("a".to_string()));
    assert_eq!(tokens[5], Token::Assign);
    let first = Token::Conditional(
        Box::new(Token::Integer(1)),
        Box::new(Token::Equals),
        Box::new(Token::FunctionCall(
            Box::new(Token::Function(
                vec![Token::Var("a".to_string())],
                vec![Token::Var("a".to_string())],
            )),
            vec![Token::Integer(1)],
        )),
    );
    let second = Token::And;
    let a2 = Token::ObjectAccess(
        Box::new(Token::Var("c".to_string())),
        Box::new(Token::Integer(0)),
    );
    let a1 = Token::ObjectAccess(Box::new(a2), Box::new(Token::Integer(0)));
    let third = Token::Conditional(
        Box::new(Token::Integer(2)),
        Box::new(Token::Equals),
        Box::new(a1),
    );
    assert_eq!(
        tokens[6],
        Token::Conditional(Box::new(first), Box::new(second), Box::new(third))
    );
    last_is_end(&tokens);
}

#[test]
fn function_in_dict() {
    let kscript = create_parser(
        "d=%[\"test\":{|d|d=(d+1);d}][\"test\"]|2|",
        VoidLogger::new(LoggerMode::Void),
    );

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0], Token::Var("d".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    let f = Token::Function(
        vec![Token::Var("d".to_string())],
        vec![
            Token::Var("d".to_string()),
            Token::Assign,
            Token::Math(vec![
                Token::Var("d".to_string()),
                Token::Addition,
                Token::Integer(1),
            ]),
            Token::End,
            Token::Var("d".to_string()),
        ],
    );
    let d = Token::Dict(vec![Token::String("test".to_string())], vec![f]);
    let a = Token::ObjectAccess(Box::new(d), Box::new(Token::String("test".to_string())));
    let c = Token::FunctionCall(Box::new(a), vec![Token::Integer(2)]);
    assert_eq!(tokens[2], c);
    last_is_end(&tokens);
}


#[test]
fn assign_loop_print() {
    let kscript = create_parser(
        "a = 1; $a<5${a = (a + 1)} a > 1",
        VoidLogger::new(LoggerMode::Void),
    );

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 9);
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
    last_is_end(&tokens);
}

#[test]
fn var_assign_var_function() {
    let kscript = create_parser(
        "a = 1; b = {|a, &e, c| e = c; d }",
        VoidLogger::new(LoggerMode::Void),
    );

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 8);
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
    );
    last_is_end(&tokens);
}

#[test]
fn basic_function_call() {
    let kscript = create_parser(
        "c = {|a| a > 1; 5}; d = c|\"test\"|",
        VoidLogger::new(LoggerMode::Void),
    );

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 8);
    assert_eq!(tokens[0], Token::Var("c".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(
        tokens[2],
        Token::Function(
            vec![Token::Var("a".to_string())],
            vec![
                Token::Var("a".to_string()),
                Token::IoWrite,
                Token::Integer(1),
                Token::End,
                Token::Integer(5),
            ],
        )
    );
    assert_eq!(tokens[3], Token::End);
    assert_eq!(tokens[4], Token::Var("d".to_string()));
    assert_eq!(tokens[5], Token::Assign);
    assert_eq!(
        tokens[6],
        Token::FunctionCall(
            Box::new(Token::Var("c".to_string())),
            vec![Token::String("test".to_string())],
        )
    );
    last_is_end(&tokens);
}

#[test]
fn anon_function_access_call() {
    let kscript = create_parser(
        "{|&c| c||}|@[{|| \"test\"}, 12][0]| >> 1",
        VoidLogger::new(LoggerMode::Void),
    );

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 4);
    assert_eq!(
        tokens[0],
        Token::FunctionCall(
            Box::new(Token::Function(
                vec![Token::Ref(Box::new(Token::Var("c".to_string())))],
                vec![
                    Token::FunctionCall(
                        Box::new(Token::Var("c".to_string())),
                        vec![]
                    ),
                ],
            )),
            vec![
                Token::ObjectAccess(
                    Box::new(Token::Array(vec![
                        Token::Function(
                            vec![],
                            vec![Token::String("test".to_string())]
                        ),
                        Token::Integer(12),
                    ])),
                    Box::new(Token::Integer(0))
                ),
            ],
        )
    );
    assert_eq!(tokens[1], Token::IoAppend);
    assert_eq!(tokens[2], Token::Integer(1));
    last_is_end(&tokens);
}

#[test]
fn reassign_array_value() {
    let kscript = create_parser(
        "a = @[1, \" \", 2]\n a[0] = \"test\"\n a >> 1",
        VoidLogger::new(LoggerMode::Void),
    );

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 12);
    assert_eq!(tokens[0], Token::Var("a".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(
        tokens[2],
        Token::Array(vec![
            Token::Integer(1),
            Token::String(" ".to_string()),
            Token::Integer(2),
        ])
    );
    assert_eq!(tokens[3], Token::End);
    assert_eq!(
        tokens[4],
        Token::ObjectAccess(
            Box::new(Token::Var("a".to_string())),
            Box::new(Token::Integer(0)),
        )
    );
    assert_eq!(tokens[5], Token::Assign);
    assert_eq!(tokens[6], Token::String("test".to_string()));
    assert_eq!(tokens[7], Token::End);
    assert_eq!(tokens[8], Token::Var("a".to_string()));
    assert_eq!(tokens[9], Token::IoAppend);
    assert_eq!(tokens[10], Token::Integer(1));
    last_is_end(&tokens);
}

#[test]
fn assign_system_command() {
    let kscript = create_parser("a = 1; \\\\1; b = 2", VoidLogger::new(LoggerMode::Void));

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 10);
    assert_eq!(tokens[0], Token::Var("a".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(tokens[2], Token::Integer(1));
    assert_eq!(tokens[3], Token::End);
    assert_eq!(tokens[4], Token::System(SystemCommand::Exit(1)));
    assert_eq!(tokens[5], Token::End);
    assert_eq!(tokens[6], Token::Var("b".to_string()));
    assert_eq!(tokens[7], Token::Assign);
    assert_eq!(tokens[8], Token::Integer(2));
    last_is_end(&tokens);
}

#[test]
fn assign_fucntion_run_output() {
    let kscript = create_parser(
        "a = {|b, c| b = @[1]; c}; a|@[\"herp\", 'derp', %[\"key\": 1]], (1 + 2 * 4)| > 1",
        VoidLogger::new(LoggerMode::Void),
    );

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 8);
    assert_eq!(tokens[0], Token::Var("a".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(
        tokens[2],
        Token::Function(
            vec![Token::Var("b".to_string()), Token::Var("c".to_string())],
            vec![
                Token::Var("b".to_string()),
                Token::Assign,
                Token::Array(vec![Token::Integer(1)]),
                Token::End,
                Token::Var("c".to_string()),
            ],
        )
    );
    assert_eq!(tokens[3], Token::End);
    assert_eq!(
        tokens[4],
        Token::FunctionCall(
            Box::new(Token::Var("a".to_string())),
            vec![
                Token::Array(vec![
                    Token::String("herp".to_string()),
                    Token::File("derp".to_string()),
                    Token::Dict(
                        vec![Token::String("key".to_string())],
                        vec![Token::Integer(1)]
                    ),
                ]),
                Token::Math(vec![
                    Token::Integer(1),
                    Token::Addition,
                    Token::Integer(2),
                    Token::Multiply,
                    Token::Integer(4),
                ]),
            ],
        )
    );
    assert_eq!(tokens[5], Token::IoWrite);
    assert_eq!(tokens[6], Token::Integer(1));
    last_is_end(&tokens);
}

#[test]
fn var_assign_access() {
    let kscript = create_parser(
        "a = @[3, 2, 1]; B = %[\"key\": \"value\"]; a[0] > 1; B[\"key\"] > 1",
        VoidLogger::new(LoggerMode::Void),
    );

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 16);
    assert_eq!(tokens[0], Token::Var("a".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(
        tokens[2],
        Token::Array(vec![
            Token::Integer(3),
            Token::Integer(2),
            Token::Integer(1),
        ])
    );
    assert_eq!(tokens[3], Token::End);
    assert_eq!(tokens[4], Token::Const("B".to_string()));
    assert_eq!(tokens[5], Token::Assign);
    assert_eq!(
        tokens[6],
        Token::Dict(
            vec![Token::String("key".to_string())],
            vec![Token::String("value".to_string())],
        )
    );
    assert_eq!(tokens[7], Token::End);
    assert_eq!(
        tokens[8],
        Token::ObjectAccess(
            Box::new(Token::Var("a".to_string())),
            Box::new(Token::Integer(0)),
        )
    );
    assert_eq!(tokens[9], Token::IoWrite);
    assert_eq!(tokens[10], Token::Integer(1));
    assert_eq!(tokens[11], Token::End);
    assert_eq!(
        tokens[12],
        Token::ObjectAccess(
            Box::new(Token::Const("B".to_string())),
            Box::new(Token::String("key".to_string())),
        )
    );
    assert_eq!(tokens[13], Token::IoWrite);
    assert_eq!(tokens[14], Token::Integer(1));
    last_is_end(&tokens);
}

#[test]
fn assign_run_io_out() {
    let kscript = create_parser(
        "a = !@[\"ls\", \"-lh\"]; a[1] > 1",
        VoidLogger::new(LoggerMode::Void),
    );

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 9);
    assert_eq!(tokens[0], Token::Var("a".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(tokens[2], Token::Run);
    assert_eq!(
        tokens[3],
        Token::Array(vec![
            Token::String("ls".to_string()),
            Token::String("-lh".to_string()),
        ])
    );
    assert_eq!(tokens[4], Token::End);
    assert_eq!(
        tokens[5],
        Token::ObjectAccess(
            Box::new(Token::Var("a".to_string())),
            Box::new(Token::Integer(1)),
        )
    );
    assert_eq!(tokens[6], Token::IoWrite);
    assert_eq!(tokens[7], Token::Integer(1));
    last_is_end(&tokens);
}

#[test]
fn take_and_set_references() {
    let kscript = create_parser(
        "a = @[1, 2, 3]; b =& a[1]; *b = 5; c = 3; b =& c; *b = 3.14",
        VoidLogger::new(LoggerMode::Void),
    );

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 26);
    assert_eq!(tokens[0], Token::Var("a".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(
        tokens[2],
        Token::Array(vec![
            Token::Integer(1),
            Token::Integer(2),
            Token::Integer(3),
        ])
    );
    assert_eq!(tokens[3], Token::End);
    assert_eq!(tokens[4], Token::Var("b".to_string()));
    assert_eq!(tokens[5], Token::TakeReference);
    assert_eq!(
        tokens[6],
        Token::ObjectAccess(
            Box::new(Token::Var("a".to_string())),
            Box::new(Token::Integer(1)),
        )
    );
    assert_eq!(tokens[7], Token::End);
    assert_eq!(tokens[8], Token::Dereference);
    assert_eq!(tokens[9], Token::Var("b".to_string()));
    assert_eq!(tokens[10], Token::Assign);
    assert_eq!(tokens[11], Token::Integer(5));
    assert_eq!(tokens[12], Token::End);
    assert_eq!(tokens[13], Token::Var("c".to_string()));
    assert_eq!(tokens[14], Token::Assign);
    assert_eq!(tokens[15], Token::Integer(3));
    assert_eq!(tokens[16], Token::End);
    assert_eq!(tokens[17], Token::Var("b".to_string()));
    assert_eq!(tokens[18], Token::TakeReference);
    assert_eq!(tokens[19], Token::Var("c".to_string()));
    assert_eq!(tokens[20], Token::End);
    assert_eq!(tokens[21], Token::Dereference);
    assert_eq!(tokens[22], Token::Var("b".to_string()));
    assert_eq!(tokens[23], Token::Assign);
    assert_eq!(tokens[24], Token::Float(3.14));
    last_is_end(&tokens);
}

#[test]
fn auto_deref_math() {
    let kscript = create_parser(
        "a = 1; b =& a; c =& b; d = (a + b + c); e = @[10, 11][c]",
        VoidLogger::new(LoggerMode::Void),
    );

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 20);
    assert_eq!(tokens[0], Token::Var("a".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(tokens[2], Token::Integer(1));
    assert_eq!(tokens[3], Token::End);
    assert_eq!(tokens[4], Token::Var("b".to_string()));
    assert_eq!(tokens[5], Token::TakeReference);
    assert_eq!(tokens[6], Token::Var("a".to_string()));
    assert_eq!(tokens[7], Token::End);
    assert_eq!(tokens[8], Token::Var("c".to_string()));
    assert_eq!(tokens[9], Token::TakeReference);
    assert_eq!(tokens[10], Token::Var("b".to_string()));
    assert_eq!(tokens[11], Token::End);
    assert_eq!(tokens[12], Token::Var("d".to_string()));
    assert_eq!(tokens[13], Token::Assign);
    assert_eq!(
        tokens[14],
        Token::Math(vec![
            Token::Var("a".to_string()),
            Token::Addition,
            Token::Var("b".to_string()),
            Token::Addition,
            Token::Var("c".to_string()),
        ])
    );
    assert_eq!(tokens[15], Token::End);
    assert_eq!(tokens[16], Token::Var("e".to_string()));
    assert_eq!(tokens[17], Token::Assign);
    assert_eq!(
        tokens[18],
        Token::ObjectAccess(
            Box::new(Token::Array(vec![Token::Integer(10), Token::Integer(11)])),
            Box::new(Token::Var("c".to_string())),
        )
    );
    last_is_end(&tokens);
}

#[test]
fn add_underscores_to_vars() {
    let kscript = create_parser("_a = 1; _1BSD = 2.21", VoidLogger::new(LoggerMode::Void));

    let tokens = get_tokens(&kscript);

    assert_eq!(tokens.len(), 8);
    assert_eq!(tokens[0], Token::Var("_a".to_string()));
    assert_eq!(tokens[1], Token::Assign);
    assert_eq!(tokens[2], Token::Integer(1));
    assert_eq!(tokens[3], Token::End);
    assert_eq!(tokens[4], Token::Const("_1BSD".to_string()));
    assert_eq!(tokens[5], Token::Assign);
    assert_eq!(tokens[6], Token::Float(2.21));
    last_is_end(&tokens);
}
