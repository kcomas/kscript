
pub struct CharContainer {
    current_chars: Vec<char>,
}

impl CharContainer {
    pub fn new() -> CharContainer {
        CharContainer { current_chars: Vec::new() }
    }

    pub fn add_char(&mut self, c: char) {
        self.current_chars.push(c);
    }

    pub fn flush(&mut self) -> String {
        let string = self.current_chars.clone().into_iter().collect();
        self.current_chars.clear();
        string
    }
}
