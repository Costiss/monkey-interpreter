use crate::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            let nth_char = self.input.chars().nth(self.read_position).unwrap();
            self.ch = Some(nth_char);
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> Option<char> {
        if self.read_position >= self.input.len() {
            return None;
        } else {
            return Some(self.input.chars().nth(self.read_position).unwrap());
        }
    }

    pub fn next_token(&mut self) -> Token {
        if self.ch.is_none() {
            return Token::new_token(TokenType::EOF, "".to_string());
        };

        self.skip_whitespace();

        let ch = self.ch.unwrap();
        let ch_string = ch.to_string();
        let token_type = match ch {
            '-' => Token::new_token(TokenType::MINUS, ch_string),
            '+' => Token::new_token(TokenType::PLUS, ch_string),
            '=' => {
                let peeked = self.peek_char().unwrap_or(' ');
                if peeked == '=' {
                    self.read_char();
                    let literal = ch_string + peeked.to_string().as_str();
                    return Token::new_token(TokenType::EQ, literal);
                }
                Token::new_token(TokenType::ASSIGN, ch_string)
            }
            '!' => {
                let peeked = self.peek_char().unwrap_or(' ');
                if peeked == '=' {
                    self.read_char();
                    let literal = ch_string + peeked.to_string().as_str();
                    return Token::new_token(TokenType::NOTEQ, literal);
                }
                Token::new_token(TokenType::BANG, ch_string)
            }
            '*' => Token::new_token(TokenType::ASTERISK, ch_string),
            '/' => Token::new_token(TokenType::SLASH, ch_string),
            '<' => Token::new_token(TokenType::LT, ch_string),
            '>' => Token::new_token(TokenType::GT, ch_string),
            ';' => Token::new_token(TokenType::SEMICOLON, ch_string),
            '(' => Token::new_token(TokenType::LPARAN, ch_string),
            ')' => Token::new_token(TokenType::RPARAN, ch_string),
            ',' => Token::new_token(TokenType::COMMA, ch_string),
            '{' => Token::new_token(TokenType::LBRACE, ch_string),
            '}' => Token::new_token(TokenType::RBRACE, ch_string),
            _ => {
                if Token::is_letter(ch) {
                    let literal = self.read_identifier();
                    let token_type = self.lookup_identifier(&literal);
                    return Token::new_token(token_type, literal);
                }
                if Token::is_digit(ch) {
                    let literal = self.read_number();
                    return Token::new_token(TokenType::INT, literal);
                }

                Token::new_token(TokenType::ILLEGAL, ch_string)
            }
        };

        self.read_char();
        return token_type;
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;

        while Token::is_letter(self.ch.unwrap()) {
            self.read_char();
        }

        self.input[position..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;

        while Token::is_digit(self.ch.unwrap()) {
            self.read_char();
        }

        self.input[position..self.position].to_string()
    }

    fn skip_whitespace(&mut self) {
        if self.ch.is_none() {
            return;
        }

        while self.ch.unwrap().is_whitespace() {
            self.read_char();
        }
    }

    fn lookup_identifier(&self, keyword: &str) -> TokenType {
        match keyword {
            "let" => TokenType::LET,
            "fn" => TokenType::FUNCTION,
            "if" => TokenType::IF,
            "else" => TokenType::ELSE,
            "return" => TokenType::RETURN,
            "true" => TokenType::TRUE,
            "false" => TokenType::FALSE,
            _ => TokenType::IDENT,
        }
    }
}
