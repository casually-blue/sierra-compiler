#[derive(Debug)]
pub struct TextLocation {
    pub line: u32,
    pub column: u32,
}

#[derive(Debug)]
pub struct TextSpan {
    pub start: TextLocation,
    pub end: TextLocation,
}
