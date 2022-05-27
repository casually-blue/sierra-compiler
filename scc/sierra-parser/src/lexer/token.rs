#[derive(Debug, Eq, PartialEq)]
pub enum Keyword {
    Use,

    Program,
    Type,
    Function,

    Begin,
    End,

    Result,

    Const,
    Mutable,
    Var,

    Public,
    Private,

    Of,
    In,
    To,
    Step,

    If,
    Then,
    Else,
    Match,

    For,
    While,
    Do,
    Loop,
    Break
}

#[derive(Debug, Eq, PartialEq)]
pub enum Literal {
    String(String),
    Number(String),
    Char(char),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Operator {
    Custom(String),
    Assignment,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ErrorToken {
    UnexpectedChar(char),
    UnknownError(String)
}

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Whitespace(String),
    Error(ErrorToken),

    Keyword(Keyword),
    Identifier(String),
    Literal(Literal),

    LeftParenthesis,
    RightParenthesis,

    LeftBrace,
    RightBrace,

    Comma,
    SemiColon,
    Colon,
    Dot,

    Assignment,
    Equals,

    EndOfFile,
}
