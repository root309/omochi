extern crate inkwell;

use crate::ast::{Expr, Operator, Statement, Type};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{FunctionValue, PointerValue};
use inkwell::IntPredicate;
use std::collections::HashMap;

// IR生成器の構造体
pub struct IRGenerator<'a> {
    context: &'a Context,
    pub module: Module<'a>,
    builder: Builder<'a>,
    variables: HashMap<String, PointerValue<'a>>,
    printf_func: Option<FunctionValue<'a>>,
}

impl<'a> IRGenerator<'a> {
    pub fn initialize_entry_block(&mut self, function: &FunctionValue) {
        let entry_block = self.context.append_basic_block(*function, "entry");
        self.builder.position_at_end(entry_block);
    }
    // Moduleの取得
    pub fn get_module(&self) -> &Module<'a> {
        &self.module
    }
    // IR生成器の新しいインスタンスを作成
    pub fn new(context: &'a Context) -> Self {
        let module = context.create_module("main");
        let builder = context.create_builder();

        IRGenerator {
            context,
            module,
            builder,
            variables: HashMap::new(), // 変数の保持用
            printf_func: None,
        }
    }
    // 関数のリターン命令を生成
    pub fn build_return(&self, value: inkwell::values::IntValue) {
        self.builder.build_return(Some(&value)).expect("Failed to build return");
    }
    fn get_printf_function(&mut self) -> FunctionValue<'a> {
        if let Some(func) = self.printf_func {
            return func;
        }

        let printf_type = self
            .context
            .i8_type()
            .ptr_type(inkwell::AddressSpace::default())
            .fn_type(
                &[self
                    .context
                    .i8_type()
                    .ptr_type(inkwell::AddressSpace::default())
                    .into()],
                true,
            );

        let printf_func = self.module.add_function("printf", printf_type, None);
        self.printf_func = Some(printf_func);
        printf_func
    }
    // StatementタイプのIRを生成するメソッド
    pub fn generate_ir_for_statement(
        &mut self,
        statement: &Statement,
        function: &FunctionValue<'a>,
    ) -> Result<inkwell::values::IntValue<'a>, ()> {
        match statement {
            Statement::Expression(expr) => Ok(self.generate_ir_inner(expr, function)),
            Statement::Declaration(name, expr) => {
                self.generate_declaration_ir(name, expr, function).unwrap();
                Ok(self.context.i32_type().const_int(0, false))
            }
            Statement::Print(expr) => {
                // printf関数の取得
                let printf_func = self.get_printf_function();
            
                // exprを評価してvalue_to_printを生成
                let value_to_print = self.generate_ir_inner(expr, function);
            
                // フォーマット文字列の定義
                let format_str = self
                    .builder
                    .build_global_string_ptr("%d\n", "fmt")
                    .expect("Failed to create format string");
            
                // printf関数の呼び出し
                self.builder.build_call(
                    printf_func,
                    &[format_str.as_pointer_value().into(), value_to_print.into()],
                    "printf_call",
                ).expect("Failed to build call");
            
                // Print文は値を返さないので、0を返す
                Ok(self.context.i32_type().const_int(0, false))
            }
            Statement::If(condition, then_branch, else_branch) => {
                // If文のIR生成ロジック
                let condition_value = self.generate_ir_inner(condition, function);
                let then_block = self.context.append_basic_block(*function, "then");
                let else_block = self.context.append_basic_block(*function, "else");
                let continue_block = self.context.append_basic_block(*function, "ifcont");

                self.builder
                    .build_conditional_branch(condition_value, then_block, else_block)
                    .expect("Failed to build conditional branch");

                self.builder.position_at_end(then_block);
                self.generate_ir_for_statement(then_branch, function)
                    .expect("Failed to generate ir for statement");
                self.builder.build_unconditional_branch(continue_block)
                    .expect("Failed to build unconditional branch");

                self.builder.position_at_end(else_block);
                if let Some(else_stmt) = else_branch {
                    self.generate_ir_for_statement(else_stmt, function)
                        .expect("Failed to generate ir for statement");
                }
                self.builder.build_unconditional_branch(continue_block)
                    .expect("Failed to build unconditional branch");

                self.builder.position_at_end(continue_block);

                Ok(self.context.i32_type().const_int(0, false))
            }
            Statement::Function(func) => {
                // 関数シグネチャの生成
                let return_type = match func.return_type {
                    Type::Int => self.context.i32_type(),
                    // 他の型の場合の処理
                };
                let fn_type = return_type.fn_type(&[], false);
                let function = self.module.add_function(&func.name, fn_type, None);

                // 関数本体の生成
                let mut last_instruction = None;
                for statement in &func.body {
                    let instruction = self.generate_ir_for_statement(statement, &function)?;
                    last_instruction = Some(instruction);
                }

                // 関数のエンドポイントの設定
                if let Some(instruction) = last_instruction {
                    self.build_return_instruction(Some(&instruction));
                } else {
                    // Void
                    self.build_return_instruction(None);
                }

                // ダミーの戻り値
                Ok(self.context.i32_type().const_int(0, false))
            }

            Statement::Block(statements) => {
                for stmt in statements {
                    self.generate_ir_for_statement(stmt, function)
                        .expect("Failed to generate ir for statement");
                }
                // ブロック自体は値を返さないので0を返す
                Ok(self.context.i32_type().const_int(0, false))
            }
            Statement::Assignment(name, expr) => {
                let value = self.generate_ir_inner(expr, function);
                let variable = self.variables.get(name).expect("Variable not found");
                self.builder.build_store(*variable, value).expect("Failed to build store");
                Ok(value)
            }
            _ => todo!("IR generation for other statement types"),
        }
    }
    fn build_return_instruction(&mut self, value: Option<&inkwell::values::IntValue<'a>>) {
        let basic_value = value.map(|v| v as &dyn inkwell::values::BasicValue);
        self.builder.build_return(basic_value).expect("Failed to build return");
    }

    // 再帰的にASTを走査してIRを生成
    fn generate_ir_inner(
        &mut self,
        expr: &Expr,
        function: &FunctionValue<'a>,
    ) -> inkwell::values::IntValue<'a> {
        match expr {
            // 整数リテラル
            Expr::Integer(value) => self.context.i32_type().const_int(*value as u64, false),
            // 二項演算
            Expr::BinaryOp(left, op, right) => {
                let left_val = self.generate_ir_inner(left, function);
                let right_val = self.generate_ir_inner(right, function);
                self.build_binary_op(left_val, right_val, op)
            }
            // 変数の参照
            Expr::Variable(name) => {
                // 変数のアドレスを取得
                let variable_address = match self.variables.get(name) {
                    Some(address) => *address,
                    None => panic!("Variable not found"),
                };

                // 変数の値をロードしてIntValueに変換
                match self.builder.build_load(variable_address, name) {
                    Ok(value) => value.into_int_value(),
                    Err(_) => panic!("Failed to load variable value"),
                }
            }
            // 変数への代入
            Expr::Assign(name, value) => {
                // 代入する値を計算
                let value_to_assign = self.generate_ir_inner(value, function);
                // 変数のアドレスを取得
                let variable_address = self.variables.get(name).expect("Variable not found");
                // 値を変数にストア
                self.builder.build_store(*variable_address, value_to_assign).expect("Failed to build store");
                value_to_assign
            }
            // if文のIR生成
            Expr::If(condition, then_branch, else_branch) => {
                // 条件、thenブロック、elseブロックの生成
                let condition_value = self.generate_ir_inner(condition, function);
                let then_block = self.context.append_basic_block(*function, "then");
                let else_block = self.context.append_basic_block(*function, "else");
                let continue_block = self.context.append_basic_block(*function, "ifcont");
                // 条件に基づいて分岐
                self.builder
                    .build_conditional_branch(condition_value, then_block, else_block)
                    .expect("Failed to build conditional branch");

                // thenブロックの生成
                self.builder.position_at_end(then_block);
                if let Statement::Block(then_statements) = &**then_branch {
                    for stmt in then_statements {
                        match stmt {
                            Statement::Expression(expr) => {
                                self.generate_ir_inner(expr, function);
                            }
                            // 他の文タイプに対するIR生成は未実装
                            _ => todo!("IR generation for other statement types"),
                        }
                    }
                }
                self.builder.build_unconditional_branch(continue_block)
                    .expect("Failed to build unconditional branch");

                // elseブロックの生成
                self.builder.position_at_end(else_block);
                if let Some(else_stmt) = else_branch {
                    if let Statement::Block(else_statements) = &**else_stmt {
                        for stmt in else_statements {
                            match stmt {
                                Statement::Expression(expr) => {
                                    self.generate_ir_inner(expr, function);
                                }
                                // 他の文タイプに対するIR生成は未実装
                                _ => todo!("IR generation for other statement types"),
                            }
                        }
                    }
                }
                self.builder.build_unconditional_branch(continue_block)
                    .expect("Failed to build unconditional branch");

                // continueブロックに移動
                self.builder.position_at_end(continue_block);

                // 一時的に0を返す
                self.context.i32_type().const_int(0, false)
            }
            // 関数呼び出しのIR生成
            Expr::FunctionCall(name, args) => {
                // 関数の検索
                let function = self.module.get_function(name).expect("Function not found");

                // 引数のIRを生成
                let mut arg_values = Vec::new();
                for arg in args {
                    let arg_value = self.generate_ir_inner(arg, &function);
                    arg_values.push(arg_value.into());
                }

                // 関数呼び出し
                match self.builder.build_call(function, &arg_values, "calltmp") {
                    Ok(call) => call.try_as_basic_value().left().unwrap().into_int_value(),
                    Err(_) => panic!("Failed to call function"),
                }
            }
        }
    }
    fn build_binary_op(
        &mut self,
        left_val: inkwell::values::IntValue<'a>,
        right_val: inkwell::values::IntValue<'a>,
        op: &Operator,
    ) -> inkwell::values::IntValue<'a> {
        match op {
            Operator::Plus => self
                .builder
                .build_int_add(left_val, right_val, "addtmp")
                .expect("Failed to add values"),
            Operator::Minus => self
                .builder
                .build_int_sub(left_val, right_val, "subtmp")
                .expect("Failed to subtract values"),
            // 等値比較のIR生成
            Operator::Equals => self
                .builder
                .build_int_compare(IntPredicate::EQ, left_val, right_val, "eqtmp")
                .expect("Failed to compare values for equality"),
            Operator::Multiply => {
                // 乗算のIRコード生成
                self.builder
                    .build_int_mul(left_val, right_val, "multmp")
                    .expect("Failed to multiply values")
            }
            Operator::MoreThan => self
                .builder
                .build_int_compare(IntPredicate::SGT, left_val, right_val, "gttmp")
                .expect("Failed to compare values"),
            Operator::LessThan => self
                .builder
                .build_int_compare(IntPredicate::SLT, left_val, right_val, "lttmp")
                .expect("Failed to compare values"),
        }
    }
    // 変数宣言のIR生成
    fn generate_declaration_ir(
        &mut self,
        name: &str,
        expr: &Expr,
        function: &FunctionValue<'a>,
    ) -> Result<(), String> {
        // 変数のアロケーションを作成
        let alloca = self.create_entry_block_alloca(function, name)?;

        // 変数をハッシュマップに登録
        self.variables.insert(name.to_string(), alloca);

        // IR値を生成
        let ir_value = self.generate_ir_inner(expr, function);

        // IR値を変数にストア
        self.builder.build_store(alloca, ir_value).expect("Failed to build store");

        Ok(())
    }

    // ブロックの先頭に変数を割り当てるための関数
    fn create_entry_block_alloca(
        &mut self,
        function: &FunctionValue,
        name: &str,
    ) -> Result<PointerValue<'a>, String> {
        let builder = self.context.create_builder();

        let entry = function
            .get_first_basic_block()
            .ok_or("No entry block in function")?;

        match entry.get_first_instruction() {
            Some(first_instr) => builder.position_before(&first_instr),
            None => builder.position_at_end(entry),
        }

        builder
            .build_alloca(self.context.i32_type(), name)
            .map_err(|e| e.to_string())
    }
}
