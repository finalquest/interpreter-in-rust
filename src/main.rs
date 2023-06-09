extern crate monkey_lib;

use std::io;

use monkey_lib::lexer::*;
use monkey_lib::lexer::token::*;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    println!("Input your code: ");
    loop {
        println!(">>>"); 
        match stdin.read_line(&mut buffer) {
            Ok(_) => {
                let mut l = Lexer::new(buffer.trim());
                loop {
                    let token: Token = l.next_token();
                    if token.tokentype == TokenTypes::EOF {
                        break;
                    }
                    println!("{{{:?}, {}}}", token.tokentype, token.literal);
                }
            }

            Err(error) => println!("error: {}", error),
        }
    }
}
