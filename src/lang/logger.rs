
use super::parser::token::Token;
use super::builder::command::Command;

#[derive(Debug, Clone)]
pub enum LoggerMode {
    Void,
    Stdout,
    Stderr,
    // filename
    File(String),
}

#[derive(Debug, Clone)]
pub enum LoggerEvent<'a, 'b, 'c, 'd, 'e, 'f> {
    ParserStart,
    // char, index, line
    ParserNextChar(char, usize, usize),
    ParserInParser(&'a str),
    ParserOutParser(&'b str),
    ParserAddToken(&'c Token),
    ParserDumpTokens(&'d Vec<Token>),
    ParserEnd,
    BuilderStart,
    BuilderCheckToken(&'e Token),
    BuilderAddCommand(&'f Command),
    BuilderEnd,
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

    fn parser_dump_tokens(&self, _tokens: &Vec<Token>) {}

    fn parser_end(&self) {}

    fn builder_start(&self) {}

    fn builder_check_token(&self, _token: &Token) {}

    fn builder_add_command(&self, _command: &Command) {}

    fn builder_end(&self) {}
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

    fn parser_dump_tokens(&self, tokens: &Vec<Token>) {
        self.write(&LoggerEvent::ParserDumpTokens(tokens));
    }

    fn parser_end(&self) {
        self.write(&LoggerEvent::ParserEnd);
    }

    fn builder_start(&self) {
        self.write(&LoggerEvent::BuilderStart);
    }

    fn builder_check_token(&self, token: &Token) {
        self.write(&LoggerEvent::BuilderCheckToken(token));
    }

    fn builder_add_command(&self, command: &Command) {
        self.write(&LoggerEvent::BuilderAddCommand(command));
    }

    fn builder_end(&self) {
        self.write(&LoggerEvent::BuilderEnd);
    }
}
