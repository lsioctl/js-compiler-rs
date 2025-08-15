mod token;
mod lexer;
mod utils;

use std::env;
use std::fs;

use crate::lexer::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");

    //let token_list = token::tokenizer(&content);

    //println!("{:?}", token_list);

    let mut lexer = Lexer::new(&content);

    // println!("position: {}, token:{:?}", lexer.position, tok)
    println!("{:?}", lexer);

    println!("{:?}", lexer.get_next_token());
    println!("{:?}", lexer.get_next_token());

}
