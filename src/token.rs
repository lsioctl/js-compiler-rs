#[derive (Debug)]
pub struct Token {
    pub kind: Kind,
    pub start: usize,
    pub end: usize,


}
#[derive (Debug, PartialEq)]
pub enum Kind {
    Unknown,
    Space,
    SemiColon,
    Dot,
    EOF,
    LeftParen,
    RightParen,
    Quote,
    DoubleQuote,
    // Identifier(String),
    // StringLitteral(String)
}

