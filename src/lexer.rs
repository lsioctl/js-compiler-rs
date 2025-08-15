use crate::token::Token;
use std::str::Chars;
use std::iter::Enumerate;

fn peek(it: &Enumerate<Chars>) -> Option<(usize, char)> {
    it.clone().next()
}


#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    chars_enumerate: Enumerate<Chars<'a>>,
    chars_offset: usize
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input,
            chars_enumerate: input.chars().enumerate(),
            chars_offset: 0
        }
    }

    pub fn get_next_token(&mut self) -> Token {
        match self.chars_enumerate.next() {
            None => {
                Token {
                    start: self.chars_offset,
                    end: self.chars_offset,
                    kind: crate::token::Kind::EOF
                }
            },
            Some((idx, c)) => {
                self.chars_offset = idx;

                println!("{}", c);

                Token {
                    start: self.chars_offset,
                    end: self.chars_offset,
                    kind: crate::token::Kind::EOF
                }                      
            }
        }
    }

    

}