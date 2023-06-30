use std::io;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod token;

fn main() {
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands");
    repl::start(io::stdin(), io::stdout());
}
