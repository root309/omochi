mod lexer;
mod parser;
mod ast;
mod interpreter;

use std::fs;
use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

fn main() {
    let filename = "program.txt";
    let input = fs::read_to_string(filename)
        .expect("Failed to read file");
    let mut lexer = Lexer::new(&input);
    let tokens = match lexer.lex() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lexer error: {:?}", e);
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse_expression() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Parser error: {:?}", e);
            return;
        }
    };

    let interpreter = Interpreter::new();
    let result = interpreter.interpret(ast);
    println!("Result: {}", result);
}
