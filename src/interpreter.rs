use crate::ast::{Expr, Operator, Statement, Function}; // ASTノード、演算子、ステートメント、関数定義を含むモジュールの使用
use std::collections::HashMap;

// インタープリタの構造体
pub struct Interpreter {
    variables: HashMap<String, i64>, // 変数の値を保持するHashMap
    functions: HashMap<String, Function>, // 関数の定義を保持するHashMap
}

impl Interpreter {
    // 新しいインタープリタのインスタンスを作成
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }
    // ステートメントの解釈
    pub fn interpret_statement(&mut self, statement: &Statement) -> i64 {
        match statement {
            Statement::Assignment(name, expr) => {
                let value = self.interpret_expression(expr);
                self.variables.insert(name.clone(), value);
                value
            },    
            Statement::Expression(expr) => self.interpret_expression(expr),
            Statement::Declaration(name, expr) => {
                let value = self.interpret_expression(expr);
                self.variables.insert(name.clone(), value);
                value
            },
            Statement::Print(expr) => {
                let value = self.interpret_expression(expr);
                println!("{}", value);
                value
            },
            Statement::Block(statements) => {
                let mut result = 0;
                for stmt in statements {
                    result = self.interpret_statement(stmt);
                }
                result
            },
            Statement::If(condition, then_branch, else_branch) => {
                let condition_val = self.interpret_expression(condition);
                if condition_val != 0 {
                    self.interpret_statement(then_branch)
                } else {
                    match else_branch {
                        Some(else_stmt) => self.interpret_statement(else_stmt),
                        None => 0,
                    }
                }
            },
            Statement::Function(function) => {
                self.functions.insert(function.name.clone(), function.clone());
                0 // 関数定義は値を返さない
            },
        }
    }
    // 式の解釈
    pub fn interpret_expression(&mut self, expr: &Expr) -> i64 {
        match expr {
            Expr::Integer(value) => *value,
            Expr::BinaryOp(left, op, right) => {
                let left_val = self.interpret_expression(left);
                let right_val = self.interpret_expression(right);
                match op {
                    Operator::Plus => left_val + right_val,
                    Operator::Minus => left_val - right_val,
                    Operator::Equals => (left_val == right_val) as i64,
                }
            },
            Expr::Variable(name) => {
                *self.variables.get(name).unwrap_or(&0)
            },
            Expr::Assign(name, value) => {
                let val = self.interpret_expression(value);
                self.variables.insert(name.clone(), val);
                val
            },
            Expr::If(condition, then_branch, else_branch) => {
                let condition_val = self.interpret_expression(condition);
                if condition_val != 0 {
                    self.interpret_statement(&*then_branch)
                } else {
                    match else_branch {
                        Some(else_stmt) => self.interpret_statement(&*else_stmt),
                        None => 0,
                    }
                }
            },
            Expr::FunctionCall(name, args) => {
                let args_vals: Vec<i64> = args.iter().map(|arg| self.interpret_expression(arg)).collect();
                // 関数呼び出しロジックはここに実装
                0 // 一時的に0を返す
            },
        }
    }
}
