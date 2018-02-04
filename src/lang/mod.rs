mod command;
mod memory;
mod data;
mod vm;
mod error;
mod util;
mod token;
mod ast;
mod symbol;
mod lexer;
mod joiner;
mod shunt;
mod builder;

use self::memory::Memory;
use self::vm::Vm;
use self::util::read_file_to_string;
use self::symbol::SymbolTable;
use self::lexer::string_to_token;
use self::joiner::join_tokens;
use self::shunt::shunt_ast;
use self::builder::build_commands;

pub fn run_parser() {
    let file_string = read_file_to_string("./examples/fib.ks").unwrap();
    println!("{}", file_string);
    let mut iter = file_string.chars().peekable();
    let token_body = string_to_token(&mut iter).unwrap();
    println!("{:#?}", token_body);
    let mut root_symbols = SymbolTable::new();
    let mut joined_ast = join_tokens(&token_body, &mut root_symbols).unwrap();
    println!("{:#?}", joined_ast);
    let shunted_ast = shunt_ast(&mut joined_ast).unwrap();
    println!("{:#?}", shunted_ast);
    let mut memory = Memory::new();
    let main_address = build_commands(&shunted_ast, 0, &mut memory, true).unwrap();
    println!("{:?}", main_address);
    println!("{:#?}", memory);

    let mut vm_calls = Vm::create_calls(main_address.get_address());

    let mut vm = Vm::new();

    let exit_code = vm.run(&mut memory, &mut vm_calls).unwrap();
    memory.clear(&main_address).unwrap();
    println!("Exit Code {}", exit_code);
    println!("{:#?}", memory);
    println!("{:#?}", vm_calls);
    println!("{:?}", vm);
}

// let mut memory = Memory::new();
//    let i1 = memory.insert(DataHolder::Integer(1));
//    let i2 = memory.insert(DataHolder::Integer(2));
//    {
//        let ref1 = memory.get(&i1);
//        let ref2 = memory.get(&i2);
//        println!("{:?}", ref1);
//        println!("{:?}", ref2);
//    }
//    memory.update(&i1, DataHolder::Integer(15));
//    let ref1 = memory.get(&i1);
//    let ref2 = memory.get(&i2);
//    println!("{:?}", ref1);
//    println!("{:?}", ref2);

//  let i1 = memory.insert(DataHolder::Integer(15), false);
//  let i2 = memory.insert(DataHolder::Integer(0), true);
//  let i3 = memory.insert(DataHolder::Integer(0), true);
//  let i4 = memory.insert(DataHolder::Integer(1), true);
//  let i5 = memory.insert(DataHolder::Integer(1), true);
//  let i6 = memory.insert(DataHolder::Integer(1), true);
//  let i7 = memory.insert(DataHolder::Integer(2), true);
//  let s1 = memory.insert(DataHolder::String("Hello World".to_string()), false);

//  let f1 = memory.insert(
//      DataHolder::Function(Function::new(
//          vec![
//              Command::LoadArgument(0),
//              Command::PushStack(i2),
//              Command::Equals,
//              Command::JumpIfFalse(2),
//              Command::PushStack(i3),
//              Command::Return,
//              Command::LoadArgument(0),
//              Command::PushStack(i4),
//              Command::Equals,
//              Command::JumpIfFalse(2),
//              Command::PushStack(i5),
//              Command::Return,
//              Command::LoadArgument(0),
//              Command::PushStack(i6),
//              Command::Sub,
//              Command::CallSelf,
//              Command::LoadArgument(0),
//              Command::PushStack(i7),
//              Command::Sub,
//              Command::CallSelf,
//              Command::Add,
//              Command::Return,
//          ],
//          1,
//      )),
//      false,
//  );

//  let main_address = memory.insert(
//      DataHolder::Function(Function::new(
//          vec![
//              Command::PushStack(i1),
//              Command::PushStack(f1),
//              Command::Call,
//              Command::PrintDebug,
//              Command::PushStack(s1),
//              Command::PrintDebug,
//              Command::Halt(0),
//          ],
//          0,
//      )),
//      false,
//  );

//  let mut vm_calls = Vm::create_calls(main_address.get_address());

//  let mut vm = Vm::new();

//  let exit_code = vm.run(&mut memory, &mut vm_calls).unwrap();
//  memory.dec(&main_address).unwrap();
//  println!("Exit Code {}", exit_code);
//  println!("{:?}", memory);
//  println!("{:?}", vm);
