mod ast;
mod irgenerator;
mod lexer;
mod parser;

use crate::irgenerator::IRGenerator;
use inkwell::context::Context;
use lexer::Lexer;
use parser::Parser;
use std::fs;

fn main() {
    let filename = "program.txt";
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let mut lexer = Lexer::new(&input);
    let tokens = match lexer.lex() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lexer error: {:?}", e);
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    let statements = match parser.parse_statements() {
        Ok(statements) => statements,
        Err(e) => {
            eprintln!("Parser error: {:?}", e);
            return;
        }
    };
    // LLVMコンテキストの作成とIRジェネレータのインスタンス化
    let context = Context::create();
    let mut ir_generator = IRGenerator::new(&context);
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = ir_generator.module.add_function("main", fn_type, None);

    // エントリブロックの初期化
    ir_generator.initialize_entry_block(&function);

    // 解析されたプログラム (AST) から LLVM IR を生成
    for statement in statements {
        ir_generator.generate_ir_for_statement(&statement, &function).expect("Failed to generate IR");
    }
    // 関数の戻り値を設定
    let return_value = context.i32_type().const_int(0, false);
    ir_generator.build_return(return_value);
    // IRをファイルに出力
    ir_generator.module.print_to_file("output.ll").unwrap();
}
