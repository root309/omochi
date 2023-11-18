use crate::ast::{Expr, Operator};

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    pub fn interpret(&self, expr: Expr) -> i64 {
        match expr {
            Expr::Integer(value) => value,
            Expr::BinaryOp(left, op, right) => {
                let left_val = self.interpret(*left);
                let right_val = self.interpret(*right);
                match op {
                    Operator::Plus => left_val + right_val,
                    Operator::Minus => left_val - right_val,
                }
            },
        }
    }
}
