// トークンの種類を定義する列挙型
#[derive(Debug, PartialEq)]
pub enum Token {
    Let,               // `let` キーワード
    Identifier(String),// 識別子
    Integer(i64),      // 整数リテラル
    Plus,              // 加算演算子
    Minus,             // 減算演算子
    Equals,            // 等号
    EOF,               // 入力の終了
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Integer(i64),
    BinaryOp(Box<Expr>, Operator, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
}

