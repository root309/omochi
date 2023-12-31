// トークンの種類を定義する列挙型
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Let,                // `let` キーワード
    Fn,                 // 関数
    If,                 // if文
    Else,               // Else
    Print,              // print文
    Identifier(String), // 識別子
    Integer(i64),       // 整数リテラル
    Plus,               // 加算演算子
    Minus,              // 減算演算子
    Equals,             // 等号
    DoubleEquals,       // 等値比較 `==`
    Asterisk,           // アスタリスク'*'
    Semicolon,          // セミコロン ;
    Colon,              // コロン :
    Comma,              // カンマ ,
    Then,               // then
    LeftParen,          // 左括弧 `(`
    RightParen,         // 右括弧 `)`
    Arrow,              // 矢印 `->`
    LeftBrace,          // 左中括弧 `{`
    RightBrace,         // 右中括弧 `}`
    MoreThan,           // 大なり >
    LessThan,           // 小なり <
    EOF,                // 入力の終了
}

// 式を表す列挙型
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Integer(i64),
    BinaryOp(Box<Expr>, Operator, Box<Expr>),
    Variable(String),
    Assign(String, Box<Expr>),
    If(Box<Expr>, Box<Statement>, Option<Box<Statement>>),
    FunctionCall(String, Vec<Expr>),
}

// 文を表す列挙型
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Expression(Expr),
    Declaration(String, Expr),
    Assignment(String, Expr),
    Print(Expr),
    Block(Vec<Statement>),
    If(Box<Expr>, Box<Statement>, Option<Box<Statement>>),
    Function(Function),
}

// 関数を表す構造体
#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub return_type: Type,
    pub body: Vec<Statement>,
    pub return_expr: Expr,
}

// 型を表す列挙型
#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Int,
}

// 演算子を表す列挙型
#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Equals,
    Multiply,
    MoreThan,
    LessThan,
}
