
#[derive(Debug, Clone)]
pub enum Event {
    // char, index, line
    NextChar(char, usize, usize),
}

pub trait Logger {
    fn new() -> Self;
    fn next_char(c: char, c_index: usize, l_index: usize) {}
}

#[derive(Debug)]
pub struct VoidLogger {}

impl Logger for VoidLogger {
    fn new() -> VoidLogger {
        VoidLogger {}
    }
}

#[derive(Debug)]
pub struct DebugLogger {
    events: Vec<Event>,
}

impl Logger for DebugLogger {
    fn new() -> DebugLogger {
        DebugLogger { events: Vec::new() }
    }
}
