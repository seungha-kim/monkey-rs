use monkey_rs::lexer::Lexer;
use monkey_rs::token::{Token, TokenType};
use std::io;
use std::io::prelude::*;

fn main() {
    const PROMPT: &str = ">> ";

    // FIXME: arbitrary Reader
    loop {
        print!("{}", PROMPT);
        std::io::stdout().flush().expect("Cannot flush stdout");
        let mut input = String::new();
        if let Ok(_) = std::io::stdin().read_line(&mut input) {
            let mut l = Lexer::new(input);
            loop {
                let tok = l.next_token();
                if tok.t == TokenType::EOF {
                    break;
                }

                println!("{:?}", tok);
            }
        } else {
            break;
        }
    }
}
