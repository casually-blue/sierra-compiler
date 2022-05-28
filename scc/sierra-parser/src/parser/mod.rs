use crate::lexer::StreamLexer;

pub struct Parser {
    lexer: StreamLexer
}

impl Parser {
    pub fn new(lexer: StreamLexer) -> Self {
        Parser {
            lexer
        }
    }
}
