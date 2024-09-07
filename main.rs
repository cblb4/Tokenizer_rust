use std::io::{self, Write};  

#[derive(Debug)]
enum TokenType {
    Word,
    Number,
    Punctuation,
    Alphanumeric
}

#[derive(Debug)]
struct Token {
    token: String,
    token_type: TokenType,
}

fn class_token(token: &str) -> TokenType {
    if token.chars().all(char::is_alphabetic) {
        TokenType::Word
    } else if token.chars().all(char::is_numeric) {
        TokenType::Number
    } else if token.chars().all(|c| !c.is_alphanumeric()) {
        TokenType::Punctuation
    } else {
        TokenType::Alphanumeric
    }
}

fn main () {
    print!("Enter a string: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); 
    let input = input.trim(); 

    print!("Enter delimiters: ");
    let mut delimiters = String::new();
    io::stdin().read_line(&mut delimiters).unwrap(); 
    let delimiters = delimiters.trim(); 

}
