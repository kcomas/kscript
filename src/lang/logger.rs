
use super::parser::token::Token;
use super::builder::command::{Command, DataHolder};
use super::vm::scope::Scope;

#[derive(Debug)]
pub enum LoggerMode {
    Void,
    Stdout,
    Stderr,
    // filename
    File(String),
}

#[derive(Debug)]
pub enum LoggerEvent<'a> {
    ParserStart,
    // char, index, line
    ParserNextChar(char, usize, usize),
    ParserInParser(&'a str),
    ParserOutParser(&'a str),
    ParserAddToken(&'a Token),
    ParserEnd,
    ParserDumpTokens(&'a Vec<Token>),
    BuilderStart,
    BuilderInBuilder(&'a str),
    BuilderOutBuilder(&'a str),
    BuilderCheckToken(&'a Token),
    BuilderAddCommand(&'a Command),
    BuilderEnd,
    BuilderDumpCommands(&'a Vec<Command>),
    ScopeEnter,
    ScopeSetRegister(usize, &'a DataHolder),
    ScopeAssign(usize, usize),
    ScopeClear,
    ScopeDump(&'a Scope),
    ScopeExit,
}

pub trait Logger {
    fn new(mode: LoggerMode) -> Self;

    fn get_mode(&self) -> &LoggerMode {
        &LoggerMode::Void
    }

    fn write(&self, event: &LoggerEvent) {
        match *self.get_mode() {
            LoggerMode::Void => {}
            LoggerMode::Stdout => println!("{:?}", event),
            LoggerMode::Stderr => eprintln!("{:?}", event),
            LoggerMode::File(_) => {}
        };
    }

    fn parser_start(&self) {}

    fn parser_next_char(&self, _c: char, _c_index: usize, _l_index: usize) {}

    fn parser_in_parser(&self, _parser_name: &str) {}

    fn parser_add_token(&self, _token: &Token) {}

    fn parser_out_parser(&self, _parser_name: &str) {}

    fn parser_end(&self) {}

    fn parser_dump_tokens(&self, _tokens: &Vec<Token>) {}

    fn builder_start(&self) {}

    fn builder_in_builder(&self, _builder_name: &str) {}

    fn builder_check_token(&self, _token: &Token) {}

    fn builder_add_command(&self, _command: &Command) {}

    fn builder_out_builder(&self, _builder_name: &str) {}

    fn builder_end(&self) {}

    fn builder_dump_commands(&self, _commands: &Vec<Command>) {}

    fn scope_enter(&self) {}

    fn scope_set_register(&self, _reg: usize, _data: &DataHolder) {}

    fn scope_assign(&self, _left: usize, _right: usize) {}

    fn scope_clear(&self) {}

    fn scope_dump(&self, _scope: &Scope) {}

    fn scope_exit(&self) {}
}

#[derive(Debug)]
pub struct VoidLogger {}

impl Logger for VoidLogger {
    fn new(_mode: LoggerMode) -> VoidLogger {
        VoidLogger {}
    }
}

#[derive(Debug)]
pub struct DebugLogger {
    mode: LoggerMode,
}

impl Logger for DebugLogger {
    fn new(mode: LoggerMode) -> DebugLogger {
        DebugLogger { mode: mode }
    }

    fn get_mode(&self) -> &LoggerMode {
        &self.mode
    }

    fn parser_start(&self) {
        self.write(&LoggerEvent::ParserStart);
    }

    fn parser_next_char(&self, c: char, c_index: usize, l_index: usize) {
        self.write(&LoggerEvent::ParserNextChar(c, c_index, l_index));
    }

    fn parser_in_parser(&self, parser_name: &str) {
        self.write(&LoggerEvent::ParserInParser(parser_name));
    }

    fn parser_add_token(&self, token: &Token) {
        self.write(&LoggerEvent::ParserAddToken(token));
    }

    fn parser_out_parser(&self, parser_name: &str) {
        self.write(&LoggerEvent::ParserOutParser(parser_name));
    }

    fn parser_end(&self) {
        self.write(&LoggerEvent::ParserEnd);
    }

    fn parser_dump_tokens(&self, tokens: &Vec<Token>) {
        self.write(&LoggerEvent::ParserDumpTokens(tokens));
    }

    fn builder_start(&self) {
        self.write(&LoggerEvent::BuilderStart);
    }

    fn builder_in_builder(&self, builder_name: &str) {
        self.write(&LoggerEvent::BuilderInBuilder(builder_name));
    }

    fn builder_check_token(&self, token: &Token) {
        self.write(&LoggerEvent::BuilderCheckToken(token));
    }

    fn builder_add_command(&self, command: &Command) {
        self.write(&LoggerEvent::BuilderAddCommand(command));
    }

    fn builder_out_builder(&self, builder_name: &str) {
        self.write(&LoggerEvent::BuilderOutBuilder(builder_name));
    }

    fn builder_end(&self) {
        self.write(&LoggerEvent::BuilderEnd);
    }

    fn builder_dump_commands(&self, commands: &Vec<Command>) {
        self.write(&LoggerEvent::BuilderDumpCommands(commands));
    }

    fn scope_enter(&self) {
        self.write(&LoggerEvent::ScopeEnter);
    }

    fn scope_set_register(&self, reg: usize, data: &DataHolder) {
        self.write(&LoggerEvent::ScopeSetRegister(reg, data));
    }

    fn scope_assign(&self, left: usize, right: usize) {
        self.write(&LoggerEvent::ScopeAssign(left, right));
    }

    fn scope_clear(&self) {
        self.write(&LoggerEvent::ScopeClear);
    }

    fn scope_dump(&self, scope: &Scope) {
        self.write(&LoggerEvent::ScopeDump(scope));
    }

    fn scope_exit(&self) {
        self.write(&LoggerEvent::ScopeExit);
    }
}
