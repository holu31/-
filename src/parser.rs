use ast::Node;

pub mod token;
pub mod lexer;
pub mod ast;

pub struct Parser {
    nodes: Vec<Node>,
    expos: usize
}

impl Parser {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            expos: 0,
        }
    }

    pub fn parse(&mut self, nodes: Vec<Node>) {
        self.nodes = nodes;

        while self.expos < self.nodes.len() {
            if let Some(Node::Statement(stat)) = self.peek(0) {
                stat.execute();
            }

            self.next();
        }
    }

    fn next(&mut self) -> Option<&Node> {
        self.expos += 1;
        self.peek(0)
    }
    
    fn peek(&mut self, add_pos: usize) -> Option<&Node> {
        if self.expos + add_pos >= self.nodes.len() {
            return None;
        }

        self.nodes.get(self.expos + add_pos)
    }
}