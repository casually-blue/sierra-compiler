use crate::Result;

use super::source_code_string::SourceCodeString;

pub trait SourceCodeText {
    fn peek(&self) -> Result<char>;
    fn pop(&mut self) -> Result<char>;
}

impl From<String> for Box<dyn SourceCodeText> {
    fn from(string: String) -> Self {
        Box::new(SourceCodeString::new(string))
    }
}

impl From<&str> for Box<dyn SourceCodeText> {
    fn from(code: &str) -> Self {
        Box::new(SourceCodeString::new(code.to_owned()))
    }
}
