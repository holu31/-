use std::collections::HashMap;
use once_cell::sync::Lazy;

use super::token::{Operator, Token, TokenType};

static OPERATORS: Lazy<HashMap<char, Operator>> = Lazy::new(|| {
    let mut operators = HashMap::new();
    operators.insert('+', Operator::Plus);
    operators.insert('-', Operator::Minus);
    operators.insert('*', Operator::Mul);

    operators
});

const WHITESPACES: [char; 4] = ['\n', '\t', ' ', '\r'];

pub struct Lexer<'a> {
    pub tokens: Vec<Token>,
    content: &'a str,
    line: usize,
    expos: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            content: "",
            line: 1,
            expos: 0,
            col: 0
        }
    }

    pub fn add_token(&mut self, token_type: TokenType) {
        let token = Token {
            token_type,
            line: self.line,
            expos: self.expos,
            col: self.col
        };
        self.tokens.push(token);
    }

    pub fn lex(&mut self, content: &'a str) {
        self.content = content;

        while self.expos < content.len() {
            let current = self.peek(0);
            if current.is_numeric() { self.number_reader() }
            else if current.is_alphabetic() || ['$', '_'].contains(&current) { self.word_reader() }
            else if OPERATORS.get(&current) != None {
                let operator_type = OPERATORS.get(&current).unwrap();
                self.add_token(TokenType::Operator(operator_type.clone()));
                self.next();
                self.col += 1;
            }
            else if WHITESPACES.contains(&current) {
                self.whitespaces_reader();
            }
            else {
                panic!("TokenException:{}:{}: unknown token", self.col, self.line);
            }
        }
    }

    fn next(&mut self) -> char {
        self.expos += 1;
        self.peek(0)
    }
    
    fn peek(&mut self, add_pos: usize) -> char {
        if self.expos + add_pos >= self.content.len() {
            return '\0';
        }

        self.content.chars()
            .nth(self.expos + add_pos)
            .unwrap()
    }

    fn number_reader(&mut self) {
        let mut buffer = String::new();
        
        let mut current = self.peek(0);
        while current.is_numeric() {
            buffer.push(current);
            current = self.next();
        }
        self.add_token(
            TokenType::Number(buffer.parse::<i32>().unwrap())
        );
        self.col += buffer.len();
    }

    fn word_reader(&mut self) {
        let mut buffer = String::new();
        
        let mut current = self.peek(0);
        while current.is_alphabetic() || current.is_numeric() || ['_'].contains(&current) {
            buffer.push(current);
            current = self.next();
        }
        self.add_token(
            TokenType::Word(buffer.clone())
        );
        self.col += buffer.len();
    }

    fn whitespaces_reader(&mut self) {
        let mut current = self.peek(0);
        while ['\n', '\t', ' ', '\r'].contains(&current) {
            match current {
                '\n' => {
                    self.col = 0;
                    self.line += 1;
                },
                _ => self.col += 1
            }

            current = self.next();
        }
    }

}