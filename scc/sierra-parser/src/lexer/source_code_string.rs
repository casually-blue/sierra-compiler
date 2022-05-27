use std::error::Error;

use super::source_code_text::SourceCodeText;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

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
    fn peek(&self) -> Result<char> {
        if let Some(c) = self.code.chars().nth(self.current) {
            Ok(c)
        } else {
            Err("EndOfFile".into())
        }
    }

    fn pop(&mut self) -> Result<char> {
        let c = self.peek();
        self.current += 1;
        c
    }
}
