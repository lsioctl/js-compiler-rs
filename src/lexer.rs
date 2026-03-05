use crate::token::{self, Token};
use std::iter::Peekable;
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

fn parse_string_litteral(pk: &mut Peekable<Chars>, separator: char) -> Result<String, &'static str> {
    let mut buf = String::new();

    let mut is_finished = false;

    for c in pk {
        // can´t use pattern matching because separator is a runtime value
        if c == separator {
            is_finished = true;
             break;
        }

        buf.push(c);
    }

    if is_finished {
        Ok(buf)
    } else {
        Err("parsing error")
    }
}

#[derive(Debug)]
pub struct Lexer<'a> {
    // input: &'a str,
    chars: Peekable<Chars<'a>>,
    offset: usize
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            // input,
            chars: input.chars().peekable(),
            offset: 0
        }
    }

    pub fn get_next_token(&mut self) -> token::Token {
        let offset = self.offset;

        let c = self.chars.next();

        let mut is_in_string_litteral = false;

        let mut string_litteral_buffer = String::new();

        if let Some(kind) = get_next_token_kind(c) {
            let next_offset = offset + 1;

            self.offset = next_offset;

            Token {
                start: offset,
                end: next_offset,
                kind
            }
        } else {
            // if let Some(c) 
            // match c {
            //     if let Some(c
            //         if ch.is_alphabetic() {
            //             if !is_in_string_litteral {
            //                 string_litteral_buffer = String::new();
            //                 is_in_string_litteral = true;

            //                 string_litteral_buffer.push(ch);
            //             }

            //             match self.chars.peek() {
            //                 Some(ch) {
            //                     match ch {

            //                     }
            //                 }
            //             }
            //         }
            //     }
            //     None => {
                    
            //     }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string_litteral_ok() {
        // I use raw sting litteral because I need double quotes
        let mut pk = r#"abc"def"#.chars().peekable();

        let sl = parse_string_litteral(&mut pk, '"').unwrap();

        assert_eq!(sl, "abc");

        assert_eq!(pk.next().unwrap(), 'd');
    }

    #[test]
    fn test_parse_string_litteral_ko() {
        // I use raw sting litteral because I need double quotes
        let mut pk = r#"abcdef"#.chars().peekable();

        let sl = parse_string_litteral(&mut pk, '"');

        assert_eq!(sl, Err("parsing error"));

        assert_eq!(pk.next(), None);
    }
}