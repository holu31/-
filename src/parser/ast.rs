// Sorry, I'm OOP kid

use core::panic;

use super::token::{Operator, Token, TokenType};

#[derive(Debug)]
pub enum Node {
    Expression(Box<dyn Expression>),
    Statement(Box<dyn Statement>)
}

#[derive(Debug)]
pub struct BinOp {
    pub operator: Token,
    pub left: Box<(dyn Expression + 'static)>,
    pub right: Box<(dyn Expression + 'static)>
}

impl Expression for BinOp {
    fn eval(&self) -> i32 {
        match self.operator.token_type {
            TokenType::Operator(Operator::Plus) => {
                self.left.eval() + self.right.eval()
            },
            TokenType::Operator(Operator::Minus) => {
                self.left.eval() - self.right.eval()
            },
            TokenType::Operator(Operator::Mul) => {
                self.left.eval() * self.right.eval()
            }
            _ => panic!("Unknown BinOp operator!")
        }
    }
}

#[derive(Debug, Clone)]
struct Number {
    token: Token
}

impl Expression for Number {
    fn eval(&self) -> i32 {
        if let TokenType::Number(n) = self.token.token_type {
            return n;
        };
        0
    }
}

pub trait Expression: std::fmt::Debug {
    fn eval(&self) -> i32;
}

#[derive(Debug)]
struct PrintStatement {
    expression: Box<(dyn Expression + 'static)>
}

impl Statement for PrintStatement {
    fn execute(&self) {
        println!("{}", self.expression.eval());
    }
}

pub trait Statement: std::fmt::Debug {
    fn execute(&self);
}

pub struct Ast {
    tokens: Vec<Token>,
    pub nodes: Vec<Node>,
    expos: usize,
}

impl Ast {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            nodes: Vec::new(),
            expos: 0
        }
    }

    pub fn build_tree(&mut self, tokens: Vec<Token>) {
        self.tokens = tokens;

        while self.expos < self.tokens.len() {
            let nodes = [
                self.expression(),
                self.statement(),
                self.syntax_invalid()
            ];

            for node in nodes.into_iter() {
                if node.is_some() {
                    self.nodes.push(node.unwrap());
                    break;
                }
            }

            self.next();
        }
    }

    fn expression(&mut self) -> Option<Node> {
        let ltoken = self.peek(0).unwrap();
        let left = Box::new(match ltoken.token_type {
            TokenType::Number(_) => Number {
                    token: ltoken
                },
            _ => return None,
        });

        if self.expos >= self.tokens.len() - 1 {
            return Some(Node::Expression(left));
        }

        let next = self.peek(1).unwrap();

        if let TokenType::Operator(_) = next.token_type {
            self.expos += 2;
            if self.peek(0).is_none() {
                panic!("ExpressionException:{}:{}: отсутствует гойда в expression",
                    next.col, next.line);
            }
            
            let right_node = self.expression();
            self.next();
            match right_node.unwrap() {
                Node::Expression(right) => {
                    return Some(Node::Expression(Box::new(BinOp {
                            operator: next,
                            left: left,
                            right: right
                        })
                    ));
                },
                _ => {}
            }
        }

        Some(Node::Expression(left))
    }

    fn statement(&mut self) -> Option<Node> {
        let token = self.peek(0).unwrap();
        if !self.equal(TokenType::Word("print".to_string())) {
            return None;
        }
        self.next();
        let expression = match self.expression() {
            Some(Node::Expression(expr)) => Some(expr),
            _ => { return None }
        };
        let statement = match token.token_type {
            TokenType::Word(_) => PrintStatement {
                expression: expression.unwrap()
            },
            _ => return None
        };

        Some(Node::Statement(Box::new(statement)))
    }

    fn syntax_invalid(&mut self) -> Option<Node> {
        match self.peek(0) {
            Some(token) => panic!("Syntax invalid:{}:{}: {} is invalid",
                token.line, token.col, token.get_token_value()),
            _ => None
        }
    }

    fn equal(&mut self, token_type: TokenType) -> bool {
        let token = self.peek(0).unwrap();
        
        token.token_type == token_type
    }

    fn next(&mut self) -> Option<Token> {
        self.expos += 1;
        self.peek(0)
    }
    
    fn peek(&mut self, add_pos: usize) -> Option<Token> {
        if self.expos + add_pos >= self.tokens.len() {
            return None;
        }

        self.tokens.get(self.expos + add_pos).cloned()
    }
}