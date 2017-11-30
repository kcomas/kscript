
extern crate kscript;

use std::collections::HashMap;
use kscript::lang::Kscript;
use kscript::lang::builder::command::{Command, DataHolder, DataType, Comparison, SystemCommand};
use kscript::lang::logger::{Logger, VoidLogger, LoggerMode};

fn create_builder<T: Logger>(program: &str, logger: T) -> Kscript<T> {
    let mut kscript = Kscript::new(logger);
    kscript.run_build_tokens_commands(program).unwrap();
    kscript
}

fn get_commands<T: Logger>(kscript: &Kscript<T>) -> &Vec<Command> {
    kscript.get_commands()
}

fn last_is_clear(commands: &Vec<Command>) {
    assert_eq!(*commands.last().unwrap(), Command::ClearRegisters);
}

#[test]
fn var_assign_integer() {
    let kscript = create_builder("test = 1234", VoidLogger::new(LoggerMode::Void));

    let commands = get_commands(&kscript);

    assert_eq!(commands.len(), 4);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Var("test".to_string()))
    );
    assert_eq!(
        commands[1],
        Command::SetRegister(1, DataHolder::Anon(DataType::Integer(1234)))
    );
    assert_eq!(commands[2], Command::Assign(0, 1));
    last_is_clear(&commands);
}

#[test]
fn constant_assign_float() {
    let kscript = create_builder("TEST = 1234.123", VoidLogger::new(LoggerMode::Void));

    let commands = get_commands(&kscript);

    assert_eq!(commands.len(), 4);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Const("TEST".to_string()))
    );
    assert_eq!(
        commands[1],
        Command::SetRegister(1, DataHolder::Anon(DataType::Float(1234.123)))
    );
    assert_eq!(commands[2], Command::Assign(0, 1));
    last_is_clear(&commands);
}

#[test]
fn var_assign_math() {
    let kscript = create_builder(
        "a = (1.234 * ((2 + 4.3) % 2) + 1 ^ 5)",
        VoidLogger::new((LoggerMode::Void)),
    );

    let commands = get_commands(&kscript);

    assert_eq!(commands.len(), 17);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Var("a".to_string()))
    );
    assert_eq!(
        commands[1],
        Command::SetRegister(1, DataHolder::Anon(DataType::Float(1.234)))
    );
    assert_eq!(
        commands[2],
        Command::SetRegister(2, DataHolder::Anon(DataType::Integer(2)))
    );
    assert_eq!(
        commands[3],
        Command::SetRegister(3, DataHolder::Anon(DataType::Float(4.3)))
    );
    assert_eq!(commands[4], Command::Addition(4, 2, 3));
    assert_eq!(commands[5], Command::SetRegister(5, DataHolder::Math(4)));
    assert_eq!(
        commands[6],
        Command::SetRegister(6, DataHolder::Anon(DataType::Integer(2)))
    );
    assert_eq!(commands[7], Command::Modulus(7, 5, 6));
    assert_eq!(commands[8], Command::SetRegister(8, DataHolder::Math(7)));
    assert_eq!(
        commands[9],
        Command::SetRegister(9, DataHolder::Anon(DataType::Integer(1)))
    );
    assert_eq!(
        commands[10],
        Command::SetRegister(10, DataHolder::Anon(DataType::Integer(5)))
    );
    assert_eq!(commands[11], Command::Exponent(11, 9, 10));
    assert_eq!(commands[12], Command::Multiply(12, 1, 8));
    assert_eq!(commands[13], Command::Addition(13, 12, 11));
    assert_eq!(commands[14], Command::SetRegister(14, DataHolder::Math(13)));
    assert_eq!(commands[15], Command::Assign(0, 14));
    last_is_clear(&commands);
}

#[test]
fn math_io_integer() {
    let kscript = create_builder("(2 * 3) > 1", VoidLogger::new(LoggerMode::Void));

    let commands = get_commands(&kscript);

    assert_eq!(commands.len(), 7);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Anon(DataType::Integer(2)))
    );
    assert_eq!(
        commands[1],
        Command::SetRegister(1, DataHolder::Anon(DataType::Integer(3)))
    );
    assert_eq!(commands[2], Command::Multiply(2, 0, 1));
    assert_eq!(commands[3], Command::SetRegister(3, DataHolder::Math(2)));
    assert_eq!(
        commands[4],
        Command::SetRegister(4, DataHolder::Anon(DataType::Integer(1)))
    );
    assert_eq!(commands[5], Command::IoWrite(3, 4));
    last_is_clear(&commands);
}

#[test]
fn math_from_access() {
    let kscript = create_builder(
        "a = @[2, 5]; b = {|| 4}; c = %[\"t\": 2, \"g\": 1]; d = (a[1] + b|| + c[\"g\"]); d >> 1",
        VoidLogger::new(LoggerMode::Void),
    );

    let commands = get_commands(&kscript);

    assert_eq!(commands.len(), 25);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Var("a".to_string()))
    );
    assert_eq!(
        commands[1],
        Command::SetRegister(
            1,
            DataHolder::Array(vec![
                DataHolder::Anon(DataType::Integer(2)),
                DataHolder::Anon(DataType::Integer(5)),
            ]),
        )
    );
    assert_eq!(commands[2], Command::Assign(0, 1));
    assert_eq!(commands[3], Command::ClearRegisters);
    assert_eq!(
        commands[4],
        Command::SetRegister(0, DataHolder::Var("b".to_string()))
    );
    assert_eq!(
        commands[5],
        Command::SetRegister(
            1,
            DataHolder::Function(
                vec![],
                vec![
                    Command::SetRegister(0, DataHolder::Anon(DataType::Integer(4))),
                ],
            ),
        )
    );
    assert_eq!(commands[6], Command::Assign(0, 1));
    assert_eq!(commands[7], Command::ClearRegisters);
    assert_eq!(
        commands[8],
        Command::SetRegister(0, DataHolder::Var("c".to_string()))
    );

    let mut map = HashMap::new();
    map.insert("t".to_string(), DataHolder::Anon(DataType::Integer(2)));
    map.insert("g".to_string(), DataHolder::Anon(DataType::Integer(1)));

    assert_eq!(commands[9], Command::SetRegister(1, DataHolder::Dict(map)));
    assert_eq!(commands[10], Command::Assign(0, 1));
    assert_eq!(commands[11], Command::ClearRegisters);
    assert_eq!(
        commands[12],
        Command::SetRegister(0, DataHolder::Var("d".to_string()))
    );
    assert_eq!(
        commands[13],
        Command::SetRegister(
            1,
            DataHolder::ObjectAccess(
                Box::new(DataHolder::Var("a".to_string())),
                Box::new(DataHolder::Anon(DataType::Integer(1))),
            ),
        )
    );
    assert_eq!(
        commands[14],
        Command::SetRegister(
            2,
            DataHolder::FunctionCall(Box::new(DataHolder::Var("b".to_string())), vec![]),
        )
    );
    assert_eq!(
        commands[15],
        Command::SetRegister(
            3,
            DataHolder::ObjectAccess(
                Box::new(DataHolder::Var("c".to_string())),
                Box::new(DataHolder::Anon(DataType::String("g".to_string()))),
            ),
        )
    );
    assert_eq!(commands[16], Command::Addition(4, 1, 2));
    assert_eq!(commands[17], Command::Addition(5, 4, 3));
    assert_eq!(commands[18], Command::SetRegister(6, DataHolder::Math(5)));
    assert_eq!(commands[19], Command::Assign(0, 6));
    assert_eq!(commands[20], Command::ClearRegisters);
    assert_eq!(
        commands[21],
        Command::SetRegister(0, DataHolder::Var("d".to_string()))
    );
    assert_eq!(
        commands[22],
        Command::SetRegister(1, DataHolder::Anon(DataType::Integer(1)))
    );
    assert_eq!(commands[23], Command::IoAppend(0, 1));
    last_is_clear(&commands);
}

#[test]
fn comment_op_comment() {
    let kscript = create_builder(
        "# this is a comment\n a = 1 # another comment",
        VoidLogger::new(LoggerMode::Void),
    );

    let commands = get_commands(&kscript);

    assert_eq!(commands.len(), 4);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Var("a".to_string()))
    );
    assert_eq!(
        commands[1],
        Command::SetRegister(1, DataHolder::Anon(DataType::Integer(1)))
    );
    assert_eq!(commands[2], Command::Assign(0, 1));
    last_is_clear(&commands);
}

#[test]
fn var_assign_file() {
    let kscript = create_builder("myfile = 'hello'", VoidLogger::new(LoggerMode::Void));

    let commands = get_commands(&kscript);

    assert_eq!(commands.len(), 4);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Var("myfile".to_string()))
    );
    assert_eq!(
        commands[1],
        Command::SetRegister(1, DataHolder::Anon(DataType::File("hello".to_string())))
    );
    assert_eq!(commands[2], Command::Assign(0, 1));
    last_is_clear(&commands);
}

#[test]
fn var_assign_string() {
    let kscript = create_builder("mystr = \"test # str\"", VoidLogger::new(LoggerMode::Void));

    let commands = get_commands(&kscript);

    assert_eq!(commands.len(), 4);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Var("mystr".to_string()))
    );
    assert_eq!(
        commands[1],
        Command::SetRegister(
            1,
            DataHolder::Anon(DataType::String("test # str".to_string())),
        )
    );
    assert_eq!(commands[2], Command::Assign(0, 1));
    last_is_clear(&commands);
}

#[test]
fn var_assign_array() {
    let kscript = create_builder(
        "a = @[1, @[1.34, \"herp\"], (1 + 2 * 3), 1234]",
        VoidLogger::new(LoggerMode::Void),
    );

    let commands = get_commands(&kscript);

    assert_eq!(commands.len(), 9);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Var("a".to_string()))
    );
    assert_eq!(
        commands[1],
        Command::SetRegister(1, DataHolder::Anon(DataType::Integer(1)))
    );
    assert_eq!(
        commands[2],
        Command::SetRegister(2, DataHolder::Anon(DataType::Integer(2)))
    );
    assert_eq!(
        commands[3],
        Command::SetRegister(3, DataHolder::Anon(DataType::Integer(3)))
    );
    assert_eq!(commands[4], Command::Multiply(4, 2, 3));
    assert_eq!(commands[5], Command::Addition(5, 1, 4));
    assert_eq!(
        commands[6],
        Command::SetRegister(
            6,
            DataHolder::Array(vec![
                DataHolder::Anon(DataType::Integer(1)),
                DataHolder::Array(vec![
                    DataHolder::Anon(DataType::Float(1.34)),
                    DataHolder::Anon(
                        DataType::String("herp".to_string())
                    ),
                ]),
                DataHolder::Math(5),
                DataHolder::Anon(DataType::Integer(1234)),
            ]),
        )
    );
    assert_eq!(commands[7], Command::Assign(0, 6));
    last_is_clear(&commands);
}

#[test]
fn var_assign_dict() {
    let kscript = create_builder(
        "d = %[\"asdf\": 1234, \"sub\": %[\"merp\": 3.45], \"arr\": @[1, 2, 4], \"herp\": \"derp\"]",
        VoidLogger::new(LoggerMode::Void),
    );

    let commands = get_commands(&kscript);

    assert_eq!(commands.len(), 4);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Var("d".to_string()))
    );

    let mut map = HashMap::new();
    let mut sub = HashMap::new();
    sub.insert("merp".to_string(), DataHolder::Anon(DataType::Float(3.45)));
    map.insert(
        "asdf".to_string(),
        DataHolder::Anon(DataType::Integer(1234)),
    );
    map.insert("sub".to_string(), DataHolder::Dict(sub));
    map.insert(
        "arr".to_string(),
        DataHolder::Array(vec![
            DataHolder::Anon(DataType::Integer(1)),
            DataHolder::Anon(DataType::Integer(2)),
            DataHolder::Anon(DataType::Integer(4)),
        ]),
    );
    map.insert(
        "herp".to_string(),
        DataHolder::Anon(DataType::String("derp".to_string())),
    );
    assert_eq!(commands[1], Command::SetRegister(1, DataHolder::Dict(map)));
    assert_eq!(commands[2], Command::Assign(0, 1));
    last_is_clear(&commands);
}

#[test]
fn var_assign_bool_const_assign_bool() {
    let kscript = create_builder("test = t; TESTD = f", VoidLogger::new(LoggerMode::Void));

    let commands = get_commands(&kscript);

    assert_eq!(commands.len(), 8);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Var("test".to_string()))
    );
    assert_eq!(
        commands[1],
        Command::SetRegister(1, DataHolder::Anon(DataType::Bool(true)))
    );
    assert_eq!(commands[2], Command::Assign(0, 1));
    assert_eq!(commands[3], Command::ClearRegisters);
    assert_eq!(
        commands[4],
        Command::SetRegister(0, DataHolder::Const("TESTD".to_string()))
    );
    assert_eq!(
        commands[5],
        Command::SetRegister(1, DataHolder::Anon(DataType::Bool(false)))
    );
    assert_eq!(commands[6], Command::Assign(0, 1));
    last_is_clear(&commands);
}

#[test]
fn vars_const_with_numbers() {
    let kscript = create_builder(
        "py3 = 3; 23a = 3.12; 1S3 = 4",
        VoidLogger::new(LoggerMode::Void),
    );

    let commands = get_commands(&kscript);

    assert_eq!(commands.len(), 12);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Var("py3".to_string()))
    );
    assert_eq!(
        commands[1],
        Command::SetRegister(1, DataHolder::Anon(DataType::Integer(3)))
    );
    assert_eq!(commands[2], Command::Assign(0, 1));
    assert_eq!(commands[3], Command::ClearRegisters);
    assert_eq!(
        commands[4],
        Command::SetRegister(0, DataHolder::Var("23a".to_string()))
    );
    assert_eq!(
        commands[5],
        Command::SetRegister(1, DataHolder::Anon(DataType::Float(3.12)))
    );
    assert_eq!(commands[6], Command::Assign(0, 1));
    assert_eq!(commands[7], Command::ClearRegisters);
    assert_eq!(
        commands[8],
        Command::SetRegister(0, DataHolder::Const("1S3".to_string()))
    );
    assert_eq!(
        commands[9],
        Command::SetRegister(1, DataHolder::Anon(DataType::Integer(4)))
    );
    last_is_clear(&commands);
}

#[test]
fn assign_conditional_true_false() {
    let kscript = create_builder(
        "?1 == 2{a = 3}{b = \"test\"}",
        VoidLogger::new(LoggerMode::Void),
    );

    let commands = get_commands(&kscript);

    assert_eq!(commands.len(), 2);

    let conditional = DataHolder::Conditional(
        Box::new(DataHolder::Anon(DataType::Integer(1))),
        Comparison::Equals,
        Box::new(DataHolder::Anon(DataType::Integer(2))),
    );

    let true_commands = vec![
        Command::SetRegister(0, DataHolder::Var("a".to_string())),
        Command::SetRegister(1, DataHolder::Anon(DataType::Integer(3))),
        Command::Assign(0, 1),
        Command::ClearRegisters,
    ];

    let false_commands =
        vec![
            Command::SetRegister(0, DataHolder::Var("b".to_string())),
            Command::SetRegister(1, DataHolder::Anon(DataType::String("test".to_string()))),
            Command::Assign(0, 1),
            Command::ClearRegisters,
        ];

    assert_eq!(
        commands[0],
        Command::If(conditional, true_commands, false_commands)
    );
    last_is_clear(&commands);
}

#[test]
fn nested_conditionial() {
    let kscript = create_builder("a = ? ?1==1^? 3== 2", VoidLogger::new(LoggerMode::Void));

    let commands = get_commands(&kscript);

    assert_eq!(commands.len(), 4);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Var("a".to_string()))
    );

    let left_conditional = DataHolder::Conditional(
        Box::new(DataHolder::Anon(DataType::Integer(1))),
        Comparison::Equals,
        Box::new(DataHolder::Anon(DataType::Integer(1))),
    );

    let right_conditional = DataHolder::Conditional(
        Box::new(DataHolder::Anon(DataType::Integer(3))),
        Comparison::Equals,
        Box::new(DataHolder::Anon(DataType::Integer(2))),
    );

    assert_eq!(
        commands[1],
        Command::SetRegister(
            1,
            DataHolder::Conditional(
                Box::new(left_conditional),
                Comparison::Or,
                Box::new(right_conditional),
            ),
        )
    );
    assert_eq!(commands[2], Command::Assign(0, 1));
    last_is_clear(&commands);
}

#[test]
fn nested_conditionals_with_nested_data() {
    let kscript = create_builder(
        "c=@[@[2]];a=??1=={|a|a}|1|&?2==c[0][0]",
        VoidLogger::new(LoggerMode::Void),
    );

    let commands = get_commands(&kscript);
    assert_eq!(commands.len(), 8);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Var("c".to_string()))
    );
    assert_eq!(
        commands[1],
        Command::SetRegister(
            1,
            DataHolder::Array(vec![
                DataHolder::Array(
                    vec![DataHolder::Anon(DataType::Integer(2))]
                ),
            ]),
        )
    );
    assert_eq!(commands[2], Command::Assign(0, 1));
    assert_eq!(commands[3], Command::ClearRegisters);
    assert_eq!(
        commands[4],
        Command::SetRegister(0, DataHolder::Var("a".to_string()))
    );
    assert_eq!(
        commands[5],
        Command::SetRegister(
            1,
            DataHolder::Conditional(
                Box::new(DataHolder::Conditional(
                    Box::new(DataHolder::Anon(DataType::Integer(1))),
                    Comparison::Equals,
                    Box::new(DataHolder::FunctionCall(
                        Box::new(DataHolder::Function(
                            vec![DataHolder::Var("a".to_string())],
                            vec![
                                Command::SetRegister(
                                    0,
                                    DataHolder::Var("a".to_string())
                                ),
                            ],
                        )),
                        vec![DataHolder::Anon(DataType::Integer(1))],
                    )),
                )),
                Comparison::And,
                Box::new(DataHolder::Conditional(
                    Box::new(DataHolder::Anon(DataType::Integer(2))),
                    Comparison::Equals,
                    Box::new(DataHolder::ObjectAccess(
                        Box::new(DataHolder::ObjectAccess(
                            Box::new(DataHolder::Var("c".to_string())),
                            Box::new(DataHolder::Anon(DataType::Integer(0))),
                        )),
                        Box::new(DataHolder::Anon(DataType::Integer(0))),
                    )),
                )),
            ),
        )
    );
    assert_eq!(commands[6], Command::Assign(0, 1));
    last_is_clear(&commands);
}

#[test]
fn function_in_dict() {
    let kscript = create_builder(
        "d=%[\"test\":{|d|d=(d+1);d}][\"test\"]|2|",
        VoidLogger::new(LoggerMode::Void),
    );

    let commands = get_commands(&kscript);

    assert_eq!(commands.len(), 4);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Var("d".to_string()))
    );

    let mut map = HashMap::new();
    map.insert(
        "test".to_string(),
        DataHolder::Function(
            vec![DataHolder::Var("d".to_string())],
            vec![
                Command::SetRegister(0, DataHolder::Var("d".to_string())),
                Command::SetRegister(1, DataHolder::Var("d".to_string())),
                Command::SetRegister(2, DataHolder::Anon(DataType::Integer(1))),
                Command::Addition(3, 1, 2),
                Command::SetRegister(4, DataHolder::Math(3)),
                Command::Assign(0, 4),
                Command::ClearRegisters,
                Command::SetRegister(0, DataHolder::Var("d".to_string())),
            ],
        ),
    );

    assert_eq!(
        commands[1],
        Command::SetRegister(
            1,
            DataHolder::FunctionCall(
                Box::new(DataHolder::ObjectAccess(
                    Box::new(DataHolder::Dict(map)),
                    Box::new(
                        DataHolder::Anon(DataType::String("test".to_string())),
                    ),
                )),
                vec![DataHolder::Anon(DataType::Integer(2))],
            ),
        )
    );
    assert_eq!(commands[2], Command::Assign(0, 1));
    last_is_clear(&commands);
}

#[test]
fn assign_loop_print() {
    let kscript = create_builder(
        "a = 1; $a<5${a = (a + 1)} a > 1",
        VoidLogger::new(LoggerMode::Void),
    );

    let commands = get_commands(&kscript);

    assert_eq!(commands.len(), 9);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Var("a".to_string()))
    );
    assert_eq!(
        commands[1],
        Command::SetRegister(1, DataHolder::Anon(DataType::Integer(1)))
    );
    assert_eq!(commands[2], Command::Assign(0, 1));
    assert_eq!(commands[3], Command::ClearRegisters);

    let conditional = DataHolder::Conditional(
        Box::new(DataHolder::Var("a".to_string())),
        Comparison::Less,
        Box::new(DataHolder::Anon(DataType::Integer(5))),
    );

    let statements = vec![
        Command::SetRegister(0, DataHolder::Var("a".to_string())),
        Command::SetRegister(1, DataHolder::Var("a".to_string())),
        Command::SetRegister(2, DataHolder::Anon(DataType::Integer(1))),
        Command::Addition(3, 1, 2),
        Command::SetRegister(4, DataHolder::Math(3)),
        Command::Assign(0, 4),
        Command::ClearRegisters,
    ];

    assert_eq!(commands[4], Command::Loop(conditional, statements));

    assert_eq!(
        commands[5],
        Command::SetRegister(0, DataHolder::Var("a".to_string()))
    );
    assert_eq!(
        commands[6],
        Command::SetRegister(1, DataHolder::Anon(DataType::Integer(1)))
    );
    assert_eq!(commands[7], Command::IoWrite(0, 1));
    last_is_clear(&commands);
}

#[test]
fn var_assign_var_function() {
    let kscript = create_builder(
        "a = 1; b = {|a, &e, c| e = c; a }",
        VoidLogger::new(LoggerMode::Void),
    );

    let commands = get_commands(&kscript);

    assert_eq!(commands.len(), 8);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Var("a".to_string()))
    );
    assert_eq!(
        commands[1],
        Command::SetRegister(1, DataHolder::Anon(DataType::Integer(1)))
    );
    assert_eq!(commands[2], Command::Assign(0, 1));
    assert_eq!(commands[3], Command::ClearRegisters);
    assert_eq!(
        commands[4],
        Command::SetRegister(0, DataHolder::Var("b".to_string()))
    );

    let args = vec![
        DataHolder::Var("a".to_string()),
        DataHolder::RefVar("e".to_string()),
        DataHolder::Var("c".to_string()),
    ];

    let statements = vec![
        Command::SetRegister(0, DataHolder::Var("e".to_string())),
        Command::SetRegister(1, DataHolder::Var("c".to_string())),
        Command::Assign(0, 1),
        Command::ClearRegisters,
        Command::SetRegister(0, DataHolder::Var("a".to_string())),
    ];

    assert_eq!(
        commands[5],
        Command::SetRegister(1, DataHolder::Function(args, statements))
    );
    assert_eq!(commands[6], Command::Assign(0, 1));
    last_is_clear(&commands);
}

#[test]
fn basic_function_call() {
    let kscript = create_builder(
        "c = {|a| a > 1; 5}; d = c|\"test\"|",
        VoidLogger::new(LoggerMode::Void),
    );

    let commands = get_commands(&kscript);
    assert_eq!(commands.len(), 8);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Var("c".to_string()))
    );

    let args = vec![DataHolder::Var("a".to_string())];

    let statements = vec![
        Command::SetRegister(0, DataHolder::Var("a".to_string())),
        Command::SetRegister(1, DataHolder::Anon(DataType::Integer(1))),
        Command::IoWrite(0, 1),
        Command::ClearRegisters,
        Command::SetRegister(0, DataHolder::Anon(DataType::Integer(5))),
    ];

    assert_eq!(
        commands[1],
        Command::SetRegister(1, DataHolder::Function(args, statements))
    );
    assert_eq!(commands[2], Command::Assign(0, 1));
    assert_eq!(commands[3], Command::ClearRegisters);
    assert_eq!(
        commands[4],
        Command::SetRegister(0, DataHolder::Var("d".to_string()))
    );
    assert_eq!(
        commands[5],
        Command::SetRegister(
            1,
            DataHolder::FunctionCall(
                Box::new(DataHolder::Var("c".to_string())),
                vec![DataHolder::Anon(DataType::String("test".to_string()))],
            ),
        )
    );
    assert_eq!(commands[6], Command::Assign(0, 1));
    last_is_clear(&commands);
}

#[test]
fn anon_function_access_call() {
    let kscript = create_builder(
        "{|&c| c||}|@[{|| \"test\"}, 12][0]| >> 1",
        VoidLogger::new(LoggerMode::Void),
    );

    let commands = get_commands(&kscript);
    assert_eq!(commands.len(), 4);
    assert_eq!(
        commands[0],
        Command::SetRegister(
            0,
            DataHolder::FunctionCall(
                Box::new(DataHolder::Function(
                    vec![DataHolder::RefVar("c".to_string())],
                    vec![
                        Command::SetRegister(
                            0,
                            DataHolder::FunctionCall(
                                Box::new(DataHolder::Var("c".to_string())),
                                vec![],
                            )
                        ),
                    ],
                )),
                vec![
                    DataHolder::ObjectAccess(
                        Box::new(DataHolder::Array(vec![
                            DataHolder::Function(
                                vec![],
                                vec![
                                    Command::SetRegister(
                                        0,
                                        DataHolder::Anon(
                                            DataType::String("test".to_string()),
                                        )
                                    ),
                                ]
                            ),
                            DataHolder::Anon(DataType::Integer(12)),
                        ])),
                        Box::new(DataHolder::Anon(DataType::Integer(0)))
                    ),
                ],
            ),
        )
    );
    assert_eq!(
        commands[1],
        Command::SetRegister(1, DataHolder::Anon(DataType::Integer(1)))
    );
    assert_eq!(commands[2], Command::IoAppend(0, 1));
    last_is_clear(&commands);
}

#[test]
fn reassign_array_value() {
    let kscript = create_builder(
        "a = @[1, \" \", 2]\n a[0] = \"test\"\n a >> 1",
        VoidLogger::new(LoggerMode::Void),
    );

    let commands = get_commands(&kscript);

    assert_eq!(commands.len(), 12);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Var("a".to_string()))
    );
    assert_eq!(
        commands[1],
        Command::SetRegister(
            1,
            DataHolder::Array(vec![
                DataHolder::Anon(DataType::Integer(1)),
                DataHolder::Anon(DataType::String(" ".to_string())),
                DataHolder::Anon(DataType::Integer(2)),
            ]),
        )
    );
    assert_eq!(commands[2], Command::Assign(0, 1));
    assert_eq!(commands[3], Command::ClearRegisters);
    assert_eq!(
        commands[4],
        Command::SetRegister(
            0,
            DataHolder::ObjectAccess(
                Box::new(DataHolder::Var("a".to_string())),
                Box::new(DataHolder::Anon(DataType::Integer(0))),
            ),
        )
    );
    assert_eq!(
        commands[5],
        Command::SetRegister(1, DataHolder::Anon(DataType::String("test".to_string())))
    );
    assert_eq!(commands[6], Command::Assign(0, 1));
    assert_eq!(commands[7], Command::ClearRegisters);
    assert_eq!(
        commands[8],
        Command::SetRegister(0, DataHolder::Var("a".to_string()))
    );
    assert_eq!(
        commands[9],
        Command::SetRegister(1, DataHolder::Anon(DataType::Integer(1)))
    );
    assert_eq!(commands[10], Command::IoAppend(0, 1));
    last_is_clear(&commands);
}

#[test]
fn assign_system_command() {
    let kscript = create_builder("a = 1; \\\\1; b = 2", VoidLogger::new(LoggerMode::Void));

    let commands = get_commands(&kscript);

    assert_eq!(commands.len(), 10);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Var("a".to_string()))
    );
    assert_eq!(
        commands[1],
        Command::SetRegister(1, DataHolder::Anon(DataType::Integer(1)))
    );
    assert_eq!(commands[2], Command::Assign(0, 1));
    assert_eq!(commands[3], Command::ClearRegisters);
    assert_eq!(
        commands[4],
        Command::SetRegister(0, DataHolder::System(SystemCommand::Exit(1)))
    );
    assert_eq!(commands[5], Command::ClearRegisters);
    assert_eq!(
        commands[6],
        Command::SetRegister(0, DataHolder::Var("b".to_string()))
    );
    assert_eq!(
        commands[7],
        Command::SetRegister(1, DataHolder::Anon(DataType::Integer(2)))
    );
    assert_eq!(commands[8], Command::Assign(0, 1));
    last_is_clear(&commands);
}

#[test]
fn assign_fucntion_run_output() {
    let kscript = create_builder(
        "a = {|b, c| b = @[1]; c}; a|@[\"herp\", 'derp', %[\"key\": 1]], (1 + 2 * 4)| > 1",
        VoidLogger::new(LoggerMode::Void),
    );

    let commands = get_commands(&kscript);
    assert_eq!(commands.len(), 13);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Var("a".to_string()))
    );
    assert_eq!(
        commands[1],
        Command::SetRegister(
            1,
            DataHolder::Function(
                vec![
                    DataHolder::Var("b".to_string()),
                    DataHolder::Var("c".to_string()),
                ],
                vec![
                    Command::SetRegister(0, DataHolder::Var("b".to_string())),
                    Command::SetRegister(
                        1,
                        DataHolder::Array(vec![DataHolder::Anon(DataType::Integer(1))])
                    ),
                    Command::Assign(0, 1),
                    Command::ClearRegisters,
                    Command::SetRegister(0, DataHolder::Var("c".to_string())),
                ],
            ),
        )
    );
    assert_eq!(commands[2], Command::Assign(0, 1));
    assert_eq!(commands[3], Command::ClearRegisters);
    assert_eq!(
        commands[4],
        Command::SetRegister(0, DataHolder::Anon(DataType::Integer(1)))
    );
    assert_eq!(
        commands[5],
        Command::SetRegister(1, DataHolder::Anon(DataType::Integer(2)))
    );
    assert_eq!(
        commands[6],
        Command::SetRegister(2, DataHolder::Anon(DataType::Integer(4)))
    );
    assert_eq!(commands[7], Command::Multiply(3, 1, 2));
    assert_eq!(commands[8], Command::Addition(4, 0, 3));

    let mut map = HashMap::new();

    map.insert("key".to_string(), DataHolder::Anon(DataType::Integer(1)));

    assert_eq!(
        commands[9],
        Command::SetRegister(
            5,
            DataHolder::FunctionCall(
                Box::new(DataHolder::Var("a".to_string())),
                vec![
                    DataHolder::Array(vec![
                        DataHolder::Anon(DataType::String("herp".to_string())),
                        DataHolder::Anon(DataType::File("derp".to_string())),
                        DataHolder::Dict(map),
                    ]),
                    DataHolder::Math(4),
                ],
            ),
        )
    );

    assert_eq!(
        commands[10],
        Command::SetRegister(6, DataHolder::Anon(DataType::Integer(1)))
    );
    assert_eq!(commands[11], Command::IoWrite(5, 6));
    last_is_clear(&commands);
}

#[test]
fn var_assign_access() {
    let kscript = create_builder(
        "a = @[3, 2, 1]; B = %[\"key\": \"value\"]; a[0] > 1; B[\"key\"] > 1",
        VoidLogger::new(LoggerMode::Void),
    );

    let commands = get_commands(&kscript);

    assert_eq!(commands.len(), 16);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Var("a".to_string()))
    );
    assert_eq!(
        commands[1],
        Command::SetRegister(
            1,
            DataHolder::Array(vec![
                DataHolder::Anon(DataType::Integer(3)),
                DataHolder::Anon(DataType::Integer(2)),
                DataHolder::Anon(DataType::Integer(1)),
            ]),
        )
    );
    assert_eq!(commands[2], Command::Assign(0, 1));
    assert_eq!(commands[3], Command::ClearRegisters);
    assert_eq!(
        commands[4],
        Command::SetRegister(0, DataHolder::Const("B".to_string()))
    );

    let mut map = HashMap::new();
    map.insert(
        "key".to_string(),
        DataHolder::Anon(DataType::String("value".to_string())),
    );
    assert_eq!(commands[5], Command::SetRegister(1, DataHolder::Dict(map)));
    assert_eq!(commands[6], Command::Assign(0, 1));
    assert_eq!(commands[7], Command::ClearRegisters);
    assert_eq!(
        commands[8],
        Command::SetRegister(
            0,
            DataHolder::ObjectAccess(
                Box::new(DataHolder::Var("a".to_string())),
                Box::new(DataHolder::Anon(DataType::Integer(0))),
            ),
        )
    );
    assert_eq!(
        commands[9],
        Command::SetRegister(1, DataHolder::Anon(DataType::Integer(1)))
    );
    assert_eq!(commands[10], Command::IoWrite(0, 1));
    assert_eq!(commands[11], Command::ClearRegisters);
    assert_eq!(
        commands[12],
        Command::SetRegister(
            0,
            DataHolder::ObjectAccess(
                Box::new(DataHolder::Const("B".to_string())),
                Box::new(DataHolder::Anon(DataType::String("key".to_string()))),
            ),
        )
    );
    assert_eq!(
        commands[13],
        Command::SetRegister(1, DataHolder::Anon(DataType::Integer(1)))
    );
    assert_eq!(commands[14], Command::IoWrite(0, 1));
    last_is_clear(&commands);
}

#[test]
fn assign_run_io_out() {
    let kscript = create_builder(
        "a = !@[\"ls\", \"-lh\"]; a[1] > 1",
        VoidLogger::new(LoggerMode::Void),
    );

    let commands = get_commands(&kscript);

    assert_eq!(commands.len(), 9);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Var("a".to_string()))
    );
    assert_eq!(
        commands[1],
        Command::SetRegister(
            1,
            DataHolder::Array(vec![
                DataHolder::Anon(DataType::String("ls".to_string())),
                DataHolder::Anon(DataType::String("-lh".to_string())),
            ]),
        )
    );
    assert_eq!(commands[2], Command::Run(2, 1));
    assert_eq!(commands[3], Command::Assign(0, 2));
    assert_eq!(commands[4], Command::ClearRegisters);
    assert_eq!(
        commands[5],
        Command::SetRegister(
            0,
            DataHolder::ObjectAccess(
                Box::new(DataHolder::Var("a".to_string())),
                Box::new(DataHolder::Anon(DataType::Integer(1))),
            ),
        )
    );
    assert_eq!(
        commands[6],
        Command::SetRegister(1, DataHolder::Anon(DataType::Integer(1)))
    );
    assert_eq!(commands[7], Command::IoWrite(0, 1));
    last_is_clear(&commands);
}

#[test]
fn take_and_set_references() {
    let kscript = create_builder(
        "a = @[1, 2, 3]; b =& a[1]; *b = 5; c = 3; b =& c; *b = 3.14",
        VoidLogger::new(LoggerMode::Void),
    );

    let commands = get_commands(&kscript);

    assert_eq!(commands.len(), 26);
    assert_eq!(
        commands[0],
        Command::SetRegister(0, DataHolder::Var("a".to_string()))
    );
    assert_eq!(
        commands[1],
        Command::SetRegister(
            1,
            DataHolder::Array(vec![
                DataHolder::Anon(DataType::Integer(1)),
                DataHolder::Anon(DataType::Integer(2)),
                DataHolder::Anon(DataType::Integer(3)),
            ]),
        )
    );
    assert_eq!(commands[2], Command::Assign(0, 1));
    assert_eq!(commands[3], Command::ClearRegisters);
    assert_eq!(
        commands[4],
        Command::SetRegister(0, DataHolder::Var("b".to_string()))
    );
    assert_eq!(
        commands[5],
        Command::SetRegister(
            1,
            DataHolder::ObjectAccess(
                Box::new(DataHolder::Var("a".to_string())),
                Box::new(DataHolder::Anon(DataType::Integer(1))),
            ),
        )
    );
    assert_eq!(commands[6], Command::TakeReference(0, 1));
    assert_eq!(commands[7], Command::ClearRegisters);
    assert_eq!(
        commands[8],
        Command::SetRegister(0, DataHolder::Var("b".to_string()))
    );
    assert_eq!(
        commands[9],
        Command::SetRegister(1, DataHolder::Anon(DataType::Integer(5)))
    );
    assert_eq!(commands[10], Command::Dereference(2, 0));
    assert_eq!(commands[11], Command::Assign(2, 1));
    assert_eq!(commands[12], Command::ClearRegisters);
    assert_eq!(
        commands[13],
        Command::SetRegister(0, DataHolder::Var("c".to_string()))
    );
    assert_eq!(
        commands[14],
        Command::SetRegister(1, DataHolder::Anon(DataType::Integer(3)))
    );
    assert_eq!(commands[15], Command::Assign(0, 1));
    assert_eq!(commands[16], Command::ClearRegisters);
    assert_eq!(
        commands[17],
        Command::SetRegister(0, DataHolder::Var("b".to_string()))
    );
    assert_eq!(
        commands[18],
        Command::SetRegister(1, DataHolder::Var("c".to_string()))
    );
    assert_eq!(commands[19], Command::TakeReference(0, 1));
    assert_eq!(commands[20], Command::ClearRegisters);
    assert_eq!(
        commands[21],
        Command::SetRegister(0, DataHolder::Var("b".to_string()))
    );
    assert_eq!(
        commands[22],
        Command::SetRegister(1, DataHolder::Anon(DataType::Float(3.14)))
    );
    assert_eq!(commands[23], Command::Dereference(2, 0));
    assert_eq!(commands[24], Command::Assign(2, 1));
    last_is_clear(&commands);
}
