
use super::parser::token::Token;

#[derive(Debug, Clone)]
pub enum LoggerMode {
    Void,
    Stdout,
    Stderr,
    // filename
    File(String),
}

#[derive(Debug, Clone)]
pub enum LoggerEvent {
    ParserStart,
    // char, index, line
    ParserNextChar(char, usize, usize),
    ParserInParser(String),
    ParserOutParser(String),
    ParserAddToken(Token),
    ParserEnd,
    BuilderStart,
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

    fn parser_start(&mut self) {}

    fn parser_next_char(&mut self, _c: char, _c_index: usize, _l_index: usize) {}

    fn parser_in_parser(&mut self, _parser_name: &str) {}

    fn parser_add_token(&mut self, _token: &Token) {}

    fn parser_out_parser(&mut self, _parser_name: &str) {}

    fn parser_end(&mut self) {}

    fn builder_start(&mut self) {}

    fn builder_end(&mut self) {}
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

    fn parser_start(&mut self) {
        self.write(&LoggerEvent::ParserStart);
    }

    fn parser_next_char(&mut self, c: char, c_index: usize, l_index: usize) {
        self.write(&LoggerEvent::ParserNextChar(c, c_index, l_index));
    }

    fn parser_in_parser(&mut self, parser_name: &str) {
        self.write(&LoggerEvent::ParserInParser(parser_name.to_string()));
    }

    fn parser_add_token(&mut self, token: &Token) {
        self.write(&LoggerEvent::ParserAddToken(token.clone()));
    }

    fn parser_out_parser(&mut self, parser_name: &str) {
        self.write(&LoggerEvent::ParserOutParser(parser_name.to_string()));
    }

    fn parser_end(&mut self) {
        self.write(&LoggerEvent::ParserEnd);
    }

    fn builder_start(&mut self) {
        self.write(&LoggerEvent::BuilderStart);
    }

    fn builder_end(&mut self) {
        self.write(&LoggerEvent::BuilderEnd);
    }
}
