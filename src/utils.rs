use std::{str::Chars, string::ParseError};


fn get_string_litteral(chars_iterator: &mut Chars, separator: char)  -> Result<String, String>
{
    let mut litteral = String::from("");
    while let Some(c) = chars_iterator.next() {
        match c {
            x if x == separator => return Ok(litteral),
            _ => litteral.push(c)
        }
    }

    Err("Parse error".to_string())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_gsl1() {
        let mut input = "aahachahaha".chars();
        assert!(get_string_litteral(&mut input, 'c') == Ok("aaha".to_string()));
    }

    #[test]
    fn test_gsl2() {
        let mut input = "aahahahaha".chars();
        assert!(get_string_litteral(&mut input, 'c') == Err("Parse error".to_string()));
    }
}