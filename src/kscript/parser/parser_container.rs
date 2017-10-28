
pub struct ParserContainer {
    text_vec: Vec<char>,
    current_char: usize,
    current_line: usize,
}

impl ParserContainer {
    pub fn new(text_str: &str) -> ParserContainer {
        ParserContainer {
            text_vec: text_str.chars().collect(),
            current_char: 0,
            current_line: 0,
        }
    }

    pub fn is_done(&self) -> bool {
        self.current_char == self.text_vec.len()
    }

    pub fn get_as_tuple(&self) -> (char, usize, usize) {
        (
            self.get_current_char(),
            self.get_current_char_index(),
            self.get_current_line_index(),
        )
    }

    pub fn get_current_char_index(&self) -> usize {
        self.current_char
    }

    pub fn get_current_line_index(&self) -> usize {
        self.current_line
    }

    pub fn get_current_char(&self) -> char {
        if self.is_done() {
            // return null byte so current parser flushes
            return '\0';
        }
        self.text_vec[self.current_char]
    }

    pub fn inc_char(&mut self) {
        self.current_char += 1;
    }

    pub fn inc_line(&mut self) {
        self.current_line += 1;
    }
}
