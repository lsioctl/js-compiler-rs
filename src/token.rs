#[derive (Debug)]
pub struct Token {
    pub kind: Kind,
    pub start: usize,
    pub end: usize,


}
#[derive (Debug)]
pub enum Kind {
    Dot,
    EOF,
    // LeftParen,
    // RightParen,
    // Quote,
    // Identifier(String)
}

