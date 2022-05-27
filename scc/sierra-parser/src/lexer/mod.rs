use std::{error::Error, fmt::Display};

use self::{token::{Token, Literal, ErrorToken, Keyword}, source_code_text::SourceCodeText};

pub mod token;
pub mod source_code_string;
pub mod source_code_text;

pub type LexerResult = Result<Token, Box<dyn Error>>;

#[derive(Debug)]
pub enum LexerError {
    EndOfCode,
    UnexpectedEscapeSequence(char),
}

impl Error for LexerError {}
impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use LexerError::*;
        match self {
            EndOfCode => write!(f, "Unexpected end of file"),
            UnexpectedEscapeSequence(c) => write!(f, "Expected escape sequence, got \\{c}")
        }
    }
}

pub struct Lexer {
    code: Box<dyn SourceCodeText>,
}

impl Lexer {
    pub fn new<T: Into<Box<dyn SourceCodeText>>>(text: T) -> Self {
        Lexer {
            code: text.into(),
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        
        while let Ok(token) = self.next_token() {
            tokens.push(token)
        }

        tokens
    }

    pub fn single_character(&mut self, constructor: fn() -> Token) -> LexerResult {
        let _  = self.code.pop()?;
        Ok(constructor())
    }

    pub fn next_token(&mut self) -> LexerResult {
        match self.code.peek()? {
            c if c.is_alphabetic() => self.parse_identifier(),
            c if c.is_numeric() => self.parse_number(),
            c if c.is_whitespace() => self.parse_whitespace(),
            '(' => self.single_character(|| Token::LeftParenthesis),
            ')' => self.single_character(|| Token::RightParenthesis),
            '[' => self.single_character(|| Token::LeftBrace),
            ']' => self.single_character(|| Token::RightBrace),
            ';' => self.single_character(|| Token::SemiColon),
            ':' => { 
                let potential = self.single_character(|| Token::Colon);
                if let Ok('=') = self.code.peek() {
                    self.single_character(|| Token::Assignment)
                } else {
                    potential
                }
            },
            '=' => self.single_character(|| Token::Equals),
            '.' => self.single_character(|| Token::Dot),
            ',' => self.single_character(|| Token::Comma),
            '*' => self.single_character(|| Token::Identifier("*".into())),
            '\'' | '\"' => self.parse_string_literal(),
            _ => Ok(Token::Error(ErrorToken::UnexpectedChar(self.code.pop()?)))
        }
    }

    pub fn parse_string_literal(&mut self) -> LexerResult {
        let start = self.code.pop()?;
        let mut string = vec![];
        while let Ok(c) = self.code.peek() && c != start {
            string.push(match c {
                '\\' => {
                    let _ = self.code.pop()?; 
                    match self.code.peek()? {
                        '\'' => '\'',
                        'n' => '\n',
                        c => return Err(box LexerError::UnexpectedEscapeSequence(c)),
                    }
                },
                _ => self.code.pop()?
            });
        }

        self.code.pop()?;

        Ok(Token::Literal(Literal::String(string.iter().collect())))
    }

    pub fn parse_with_predicate(&mut self, predicate: fn(char) -> bool, constructor: fn(String) -> Token) -> LexerResult {
        let mut token = vec![];
        while let Ok(c) = self.code.peek() && predicate(c) {
            token.push(self.code.pop()?);
        }

        Ok(constructor(token.iter().collect()))
    }

    pub fn parse_identifier(&mut self) -> LexerResult {
        let identifier = self.parse_with_predicate(|c| c.is_alphanumeric() || c == '_', Token::Identifier)?;
        Ok(if let Token::Identifier(keyword) = identifier {
            match keyword.as_ref() {
                "use" => Token::Keyword(Keyword::Use),
                "begin" => Token::Keyword(Keyword::Begin),
                "end" => Token::Keyword(Keyword::End),
                _ => Token::Identifier(keyword)
            }

        } else {
            identifier
        })
    }

    pub fn parse_number(&mut self) -> LexerResult {
        self.parse_with_predicate(|c| c.is_numeric() || c == '_', |s| Token::Literal(Literal::Number(s)))
    }

    pub fn parse_whitespace(&mut self) -> LexerResult {
        self.parse_with_predicate(|c| c.is_whitespace(), Token::Whitespace)
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::{token::{Token, Literal}, Lexer};

    #[test]
    fn test_identifier() {
        assert_eq!(Lexer::new("test").next_token().unwrap(), Token::Identifier("test".into()));
        assert_eq!(Lexer::new("identifier   ").next_token().unwrap(), Token::Identifier("identifier".into()));
    }

    #[test]
    fn test_number_literal() {
        assert_eq!(Lexer::new("1234").next_token().unwrap(), Token::Literal(Literal::Number("1234".into())));
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(Lexer::new(" \n\t").next_token().unwrap(), Token::Whitespace(" \n\t".into()));
    }

    #[test]
    fn test_multiple() {
        assert_eq!(Lexer::new("test 99").lex(), vec![Token::Identifier("test".into()), Token::Whitespace(" ".into()), Token::Literal(Literal::Number("99".into()))])
    }
}
