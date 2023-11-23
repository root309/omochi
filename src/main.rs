mod ast;
mod interpreter;
mod irgenerator;
mod lexer;
mod parser;

use crate::irgenerator::IRGenerator;
use inkwell::context::Context;
use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
use std::fs;

fn main() {
    // 読み込むソースファイルの名前
    let filename = "program.txt";
    // ファイルからソースコードを読み込む
    let input = fs::read_to_string(filename).expect("Failed to read file");

    // 字句解析器のインスタンス化と字句解析の実行
    let mut lexer = Lexer::new(&input);
    let tokens = match lexer.lex() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lexer error: {:?}", e);
            return;
        }
    };

    // 構文解析器のインスタンス化とASTの構築
    let mut parser = Parser::new(tokens);
    let statements = match parser.parse_block() {
        Ok(statements) => statements,
        Err(e) => {
            eprintln!("Parser error: {:?}", e);
            return;
        }
    };

    // LLVMコンテキストの作成とIRジェネレータのインスタンス化
    let context = Context::create();
    let ir_generator = IRGenerator::new(&context);
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = ir_generator.module.add_function("main", fn_type, None);

    // 解析されたプログラム (AST) から LLVM IR を生成
    for statement in statements {
        ir_generator.generate_ir_for_statement(&statement, &function);
    }

    // IRをファイルに出力
    ir_generator.module.print_to_file("output.ll").unwrap();
}
