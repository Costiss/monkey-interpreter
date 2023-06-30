use std::io::{self, BufRead, BufReader, Write};

use crate::lexer::Lexer;
use crate::token::TokenType;

const PROMPT: &'static str = ">> ";

pub fn start(input: io::Stdin, mut output: impl Write) {
    let mut scanner = BufReader::new(input);

    loop {
        write!(output, "{}", PROMPT).unwrap();
        output.flush().unwrap();

        let mut line = String::new();
        let scanned = scanner.read_line(&mut line).unwrap();
        if scanned == 0 {
            return;
        }

        let mut lexer = Lexer::new(line.trim().to_string());

        loop {
            let token = lexer.next_token();

            if token.token_type == TokenType::EOF {
                break;
            };
            write!(output, "{:?}\n", token).unwrap();
            output.flush().unwrap();
        }
    }
}
