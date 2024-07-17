use parser::{ast::Ast, lexer::Lexer, Parser};

pub mod parser;

fn main() {
    let content = "print 2+2*2";
    let mut lexer: Lexer = Lexer::new();
    lexer.lex(content);

    println!("{:#?}", lexer.tokens);

    let mut ast: Ast = Ast::new();
    ast.build_tree(lexer.tokens);

    println!("{:#?}", ast.nodes);

    let mut parser = Parser::new();
    parser.parse(ast.nodes);
}
