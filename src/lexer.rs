use crate::token::{self, Token};
use std::str::Chars;

// fn peek(it: &Chars) -> Option<char> {
//     it.clone().next()
// }


fn get_next_token_kind(c: Option<char>) -> Option<token::Kind> {
    match c {
        None => Some(token::Kind::EOF),
        Some(c) => {
            match c {
                ';' => Some(token::Kind::SemiColon),
                '.' => Some(token::Kind::Dot),
                '(' => Some(token::Kind::LeftParen),
                ')' => Some(token::Kind::RightParen),
                '"' => Some(token::Kind::DoubleQuote),
                '\'' => Some(token::Kind::Quote),
                _ => None
            }
        }
    }
}

#[derive(Debug)]
pub struct Lexer<'a> {
    // input: &'a str,
    chars: Chars<'a>,
    offset: usize
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            // input,
            chars: input.chars(),
            offset: 0
        }
    }

    pub fn get_next_token(&mut self) -> token::Token {
        let offset = self.offset;

        if let Some(kind) = get_next_token_kind(self.chars.next()) {
            let next_offset = offset + 1;

            self.offset = next_offset;

            Token {
                start: offset,
                end: next_offset,
                kind
            }
        } else {
            // TODO dummy implementation for now

            let next_offset = offset + 1;

            self.offset = next_offset;
            Token {
                start: offset,
                end: next_offset,
                kind: token::Kind::Unknown
            }
        }
    }

}