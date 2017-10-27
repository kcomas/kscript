
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
    ParserAddToken(Token),
    ParserEnd,
}

pub trait Logger {
    fn new(mode: LoggerMode) -> Self;

    fn get_mode(&self) -> &LoggerMode {
        &LoggerMode::Void
    }

    fn add_event(&mut self, event: LoggerEvent) {}

    fn get_last_event(&self) -> Option<&LoggerEvent> {
        None
    }

    fn write(&self) {
        if let LoggerMode::Void = *self.get_mode() {
            return;
        }

        if let Some(ref event) = self.get_last_event() {
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

    fn parser_next_char(&mut self, c: char, c_index: usize, l_index: usize) {}

    fn parser_add_token(&mut self, token: Token) {}

    fn parser_end(&mut self) {}
}

#[derive(Debug)]
pub struct VoidLogger {}

impl Logger for VoidLogger {
    fn new(mode: LoggerMode) -> VoidLogger {
        VoidLogger {}
    }
}

#[derive(Debug)]
pub struct DebugLogger {
    mode: LoggerMode,
    events: Vec<LoggerEvent>,
}

impl Logger for DebugLogger {
    fn new(mode: LoggerMode) -> DebugLogger {
        DebugLogger {
            mode: mode,
            events: Vec::new(),
        }
    }

    fn get_mode(&self) -> &LoggerMode {
        &self.mode
    }

    fn add_event(&mut self, event: LoggerEvent) {
        self.events.push(event);
    }

    fn get_last_event(&self) -> Option<&LoggerEvent> {
        self.events.last()
    }

    fn parser_start(&mut self) {
        self.add_event(LoggerEvent::ParserStart);
        self.write();
    }

    fn parser_add_token(&mut self, token: Token) {
        self.add_event(LoggerEvent::ParserAddToken(token));
        self.write();
    }

    fn parser_end(&mut self) {
        self.add_event(LoggerEvent::ParserEnd);
        self.write();
    }
}
