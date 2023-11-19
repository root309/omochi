extern crate inkwell;

use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::module::Module;
use crate::ast::{Expr, Operator};

pub struct IRGenerator<'a> {
    context: &'a Context,
    pub module: Module<'a>,
    builder: Builder<'a>,
}

impl<'a> IRGenerator<'a> {
    pub fn get_module(&self) -> &Module<'a> {
        &self.module
    }
    pub fn new(context: &'a Context) -> Self {
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
                    Operator::Plus => self.builder.build_int_add(left_val, right_val, "tmpadd").expect("Failed to add values"),
                    Operator::Minus => self.builder.build_int_sub(left_val, right_val, "tmpsub").expect("Failed to subtract values"),
                }
            }
        }
    }
}
