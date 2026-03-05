mod token;
mod lexer;
mod utils;

use std::env;
use std::fs;

use crate::lexer::Lexer;

fn main() {
    
    let input = "let x = 42;";
    let mut lexer = Lexer::new(input);

    loop {
        let token = lexer.next_token();
        println!("{:?} at start: {:?} - end: {:?}", token.kind, token.start, token.end);
        if token.kind ==lexer::TokenKind::EOF {
            break;
        }
    }

}
