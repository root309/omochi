extern crate inkwell;

use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::module::Module;
use crate::ast::{Expr, Operator};

pub struct IRGenerator {
    context: Context,
    module: Module,
    builder: Builder,
}

impl IRGenerator {
    pub fn new() -> Self {
        let context = Context::create();
        let module = context.create_module("main");
        let builder = context.create_builder();

        IRGenerator { context, module, builder }
    }

    pub fn generate_ir(&self, expr: &Expr) -> inkwell::values::IntValue {
        match expr {
            Expr::Integer(value) => self.context.i32_type().const_int(*value as u64, false),
            Expr::BinaryOp(left, op, right) => {
                let left_val = self.generate_ir(left);
                let right_val = self.generate_ir(right);

                match op {
                    Operator::Plus => self.builder.build_int_add(left_val, right_val, "tmpadd"),
                    Operator::Minus => self.builder.build_int_sub(left_val, right_val, "tmpsub"),
                }
            }
        }
    }
}
