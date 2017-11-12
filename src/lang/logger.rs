
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

    fn set_event(&mut self, _event: LoggerEvent) {}

    fn get_last_event(&self) -> &Option<LoggerEvent> {
        &None
    }

    fn write(&self) {
        if let LoggerMode::Void = *self.get_mode() {
            return;
        }

        if let Some(ref event) = *self.get_last_event() {
            match *self.get_mode() {
                LoggerMode::Void => {}
                LoggerMode::Stdout => println!("{:?}", event),
                LoggerMode::Stderr => eprintln!("{:?}", event),
                LoggerMode::File(_) => {}
            };
            return;
        }
        panic!("Logger Write Called And No LoggerEvent Found")
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
    last_event: Option<LoggerEvent>,
}

impl Logger for DebugLogger {
    fn new(mode: LoggerMode) -> DebugLogger {
        DebugLogger {
            mode: mode,
            last_event: None,
        }
    }

    fn get_mode(&self) -> &LoggerMode {
        &self.mode
    }

    fn set_event(&mut self, event: LoggerEvent) {
        self.last_event = Some(event);
    }

    fn get_last_event(&self) -> &Option<LoggerEvent> {
        &self.last_event
    }

    fn parser_start(&mut self) {
        self.set_event(LoggerEvent::ParserStart);
        self.write();
    }

    fn parser_next_char(&mut self, c: char, c_index: usize, l_index: usize) {
        self.set_event(LoggerEvent::ParserNextChar(c, c_index, l_index));
        self.write();
    }

    fn parser_in_parser(&mut self, parser_name: &str) {
        self.set_event(LoggerEvent::ParserInParser(parser_name.to_string()));
        self.write();
    }

    fn parser_add_token(&mut self, token: &Token) {
        self.set_event(LoggerEvent::ParserAddToken(token.clone()));
        self.write();
    }

    fn parser_out_parser(&mut self, parser_name: &str) {
        self.set_event(LoggerEvent::ParserOutParser(parser_name.to_string()));
        self.write();
    }

    fn parser_end(&mut self) {
        self.set_event(LoggerEvent::ParserEnd);
        self.write();
    }

    fn builder_start(&mut self) {
        self.set_event(LoggerEvent::BuilderStart);
        self.write();
    }

    fn builder_end(&mut self) {
        self.set_event(LoggerEvent::BuilderEnd);
        self.write();
    }
}
