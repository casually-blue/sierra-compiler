use self::{token::{Token, Literal, ErrorToken}, source_code_text::SourceCodeText};

pub mod token;
pub mod source_code_string;
pub mod source_code_text;

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
        
        while let Some(token) = self.next_token() {
            tokens.push(token)
        }

        tokens
    }

    pub fn next_token(&mut self) -> Option<Token> {
        match self.code.peek()? {
            c if c.is_alphabetic() => self.parse_identifier(),
            c if c.is_numeric() => self.parse_number(),
            c if c.is_whitespace() => self.parse_whitespace(),
            _ => Some(Token::Error(ErrorToken::UnexpectedChar(self.code.pop()?)))
        }
    }

    pub fn parse_with_predicate(&mut self, predicate: fn(char) -> bool, constructor: fn(String) -> Token) -> Option<Token> {
        let mut token = vec![];
        while let Some(c) = self.code.peek() && predicate(c) {
            token.push(self.code.pop()?);
        }

        Some(constructor(token.iter().collect()))
    }

    pub fn parse_identifier(&mut self) -> Option<Token> {
        self.parse_with_predicate(|c| !(c.is_whitespace() || c.is_numeric()), Token::Identifier)
    }

    pub fn parse_number(&mut self) -> Option<Token> {
        self.parse_with_predicate(|c| c.is_numeric() || c == '_', |s| Token::Literal(Literal::Number(s)))
    }

    pub fn parse_whitespace(&mut self) -> Option<Token> {
        self.parse_with_predicate(|c| c.is_whitespace(), Token::Whitespace)
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::{token::{Token, Literal}, Lexer};

    #[test]
    fn test_identifier() {
        assert_eq!(Lexer::new("test").next_token(), Some(Token::Identifier("test".into())));
        assert_eq!(Lexer::new("identifier   ").next_token(), Some(Token::Identifier("identifier".into())));
    }

    #[test]
    fn test_number_literal() {
        assert_eq!(Lexer::new("1234").next_token(), Some(Token::Literal(Literal::Number("1234".into()))));
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(Lexer::new(" \n\t").next_token(), Some(Token::Whitespace(" \n\t".into())));
    }

    #[test]
    fn test_multiple() {
        assert_eq!(Lexer::new("test 99").lex(), vec![Token::Identifier("test".into()), Token::Whitespace(" ".into()), Token::Literal(Literal::Number("99".into()))])
    }
}
