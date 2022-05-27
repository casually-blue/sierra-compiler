use super::source_code_text::SourceCodeText;

pub struct SourceCodeString {
    code: String,
    current: usize,
}

impl SourceCodeString {
    pub fn new(code: String) -> Self {
        SourceCodeString {
            code,
            current: 0
        }
    }
}

impl SourceCodeText for SourceCodeString {
    fn peek(&self) -> Option<char> {
        self.code.chars().nth(self.current)
    }

    fn pop(&mut self) -> Option<char> {
        self.current += 1;
        self.code.chars().nth(self.current-1)
    }
}
