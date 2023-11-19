mod lexer;
mod parser;
mod ast;
mod interpreter;
mod irgenerator;

use std::fs;
use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;
use crate::irgenerator::IRGenerator;
use inkwell::context::Context;

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

    let context = Context::create();
    let ir_generator = IRGenerator::new(&context);
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = ir_generator.module.add_function("main", fn_type, None);

    // ASTからLLVM IRを生成し、関数の戻り値を設定
    let ir_value = ir_generator.generate_ir(&ast, &function);
    ir_generator.build_return(ir_value);

    // IRをファイルに出力
    ir_generator.module.print_to_file("output.ll").unwrap();
    //println!("Result: {}", interpreted_result);
}
