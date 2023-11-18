mod lexer;
mod parser;
mod ast;
use lexer::Lexer;
use parser::Parser;

fn main() {
    let input = "let x = 5 + 10;";
    let mut lexer = Lexer::new(input);
    match lexer.lex() {
        Ok(tokens) => {
            let mut parser = Parser::new(tokens);
            match parser.parse_expression() {
                Ok(ast) => println!("AST: {:?}", ast),
                Err(e) => eprintln!("Parser error: {:?}", e),
            }
        }
        Err(e) => eprintln!("Lexer error: {:?}", e),
    }
}