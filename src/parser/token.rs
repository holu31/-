#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Number(i32),
    Word(String),
    Operator(Operator)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Mul,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub expos: usize,
    pub col: usize
}

impl Token {
    pub fn get_token_value(&self) -> String {
        match &self.token_type {
            TokenType::Number(num) => num.to_string(),
            TokenType::Word(word) => word.clone(),
            TokenType::Operator(op) => match op {
                Operator::Plus => "+".to_string(),
                Operator::Minus => "-".to_string(),
                Operator::Mul => "*".to_string(),
            },
        }
    }
}