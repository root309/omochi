use crate::ast::{Expr, Function, Operator, Statement, Token, Type};

// 構文解析器のエラーを表す列挙型
#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken { expected: String, found: String },
    UnexpectedEOF,
    InvalidSyntax,
}

// 構文解析器本体の構造体
pub struct Parser {
    tokens: Vec<Token>, // 解析するトークンの列
    current: usize,     // 現在解析中のトークンの位置
}

impl Parser {
    // 新しい構文解析器インスタンスを作成
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }
    // カーソルを進める補助関数
    fn advance(&mut self) {
        self.consume();
    }

    // 直前のトークンを取得する補助関数
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
    // トークン列から次のトークンを取得し、カーソルを進める
    fn consume(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.current)?.clone();
        println!("Consuming token: {:?}", token);
        self.current += 1;
        Some(token)
    }

    // 現在のトークンを取得(消費しない)
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }
    fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.current + 1)
    }
    // 現在のトークンが指定したトークンかどうかを確認
    fn check(&self, token: &Token) -> bool {
        match (self.peek(), token) {
            (Some(current_token), expected_token) => current_token == expected_token,
            _ => false,
        }
    }
    // 指定したトークンが現在のトークンであれば、それを消費して true を返す
    fn match_token(&mut self, token: Token) -> bool {
        println!("Checking token: {:?}, Current token: {:?}", token, self.peek());
        if let Some(current_token) = self.peek() {
            match (&token, current_token) {
                (Token::Else, Token::Identifier(name)) if name == "else" => {
                    self.consume();
                    true
                },
                _ if &token == current_token => {
                    self.consume();
                    true
                },
                _ => {
                    println!("Token not matched: {:?}, Current token: {:?}", token, self.peek());
                    false
                }
            }
        } else {
            false
        }
    }
    
    
    // 指定されたトークンを期待しているか確認し、そうでなければエラー
    fn expect_token(&mut self, expected: Token) -> Result<(), ParserError> {
        let token = self.consume().ok_or(ParserError::UnexpectedEOF)?;
        if token != expected {
            return Err(ParserError::UnexpectedToken {
                expected: format!("{:?}", expected),
                found: format!("{:?}", token),
            });
        }
        Ok(())
    }

    // 現在のトークンがEOF(入力終了)かどうか
    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Some(Token::EOF) | None)
    }
    // 問題ではなさそう
    // 式の解析
    pub fn parse_expression(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.parse_primary()?;

        while let Some(op) = self.next_operator()? {
            match op {
                Operator::Equals => {
                    if let Expr::Variable(name) = expr {
                        println!("Parsing assignment, current token: {:?}", self.peek());
                        self.consume(); // '=' トークンを消費する
                        let rhs = self.parse_expression()?; // 右辺の式を解析
                        expr = Expr::Assign(name, Box::new(rhs));
                    } else {
                        return Err(ParserError::InvalidSyntax);
                    }
                }
                _ => {
                    let rhs = self.parse_primary()?;
                    expr = Expr::BinaryOp(Box::new(expr), op, Box::new(rhs));
                }
            }
        }

        Ok(expr)
    }

    // 単項式(数字など基本的な要素)の解析
    fn parse_primary(&mut self) -> Result<Expr, ParserError> {
        match self.peek().cloned() {
            Some(Token::Integer(value)) => {
                self.consume();
                Ok(Expr::Integer(value))
            }
            Some(Token::LeftParen) => {
                self.consume(); // 左括弧を消費
                let expr = self.parse_expression()?; // 括弧内の式を解析
                self.expect_token(Token::RightParen)?; // 対応する右括弧を期待
                Ok(expr)
            }
            Some(Token::Identifier(ref name)) if self.peek_next() == Some(&Token::LeftParen) => {
                self.consume(); // 関数名を消費
                self.consume(); // 左括弧を消費
                self.expect_token(Token::RightParen)?; // 対応する右括弧を期待
                Ok(Expr::FunctionCall(name.clone(), vec![]))
            }
            Some(Token::Identifier(name)) => {
                self.consume();
                Ok(Expr::Variable(name))
            }
            _ => Err(ParserError::UnexpectedToken {
                expected: String::from("Integer, LeftParen, Identifier, or FunctionCall"),
                found: format!("{:?}", self.peek()),
            }),
        }
    }

    // 次の演算子を取得
    fn next_operator(&mut self) -> Result<Option<Operator>, ParserError> {
        let operator = match self.tokens.get(self.current) {
            Some(Token::Plus) => {
                self.advance();
                Some(Operator::Plus)
            }
            Some(Token::Minus) => {
                self.advance();
                Some(Operator::Minus)
            }
            Some(Token::MoreThan) => {
                self.consume();
                Some(Operator::MoreThan)
            }
            Some(Token::LessThan) => {
                self.consume();
                Some(Operator::LessThan)
            }
            Some(Token::Asterisk) => {
                self.consume();
                Some(Operator::Multiply)
            }
            _ => None,
        };
        Ok(operator)
    }

    // 変数宣言の解析
    fn parse_declaration(&mut self) -> Result<Statement, ParserError> {
        self.expect_token(Token::Let)?;
        let name = match self.consume() {
            Some(Token::Identifier(name)) => name,
            Some(found) => {
                return Err(ParserError::UnexpectedToken {
                    expected: format!("{:?}", Token::Identifier("".to_string())),
                    found: format!("{:?}", found),
                })
            }
            None => return Err(ParserError::UnexpectedEOF),
        };
        self.expect_token(Token::Equals)?;
        let expr = self.parse_expression()?;
        Ok(Statement::Declaration(name, expr))
    }

    // 関数定義の解析
    fn parse_function(&mut self) -> Result<Function, ParserError> {
        self.expect_token(Token::Fn)?;
        let name = self.parse_identifier()?;
        self.expect_token(Token::LeftParen)?;
        let params = self.parse_parameters()?;
        self.expect_token(Token::RightParen)?;
        self.expect_token(Token::Arrow)?;
        let return_type = self.parse_type()?;
        let body = self.parse_block()?;
        Ok(Function {
            name,
            params,
            return_type,
            body,
        })
    }
    // print文の解析
    fn parse_print_statement(&mut self) -> Result<Statement, ParserError> {
        self.expect_token(Token::Print)?;
        let expr = self.parse_expression()?;
        Ok(Statement::Print(expr))
    }
    // 正しい挙動
    // 代入または式の文の解析
    fn parse_assignment_or_expression_statement(&mut self) -> Result<Statement, ParserError> {
        let left = self.parse_expression()?; // 左辺の式を解析
        if let Some(Token::Equals) = self.peek() {
            // Equals トークンがある場合は代入文として処理
            self.consume(); // Equals トークンを消費
            let right = self.parse_expression()?; // 右辺の式を解析
            self.expect_token(Token::Semicolon)?; // 代入文の後にセミコロンを期待
            if let Expr::Variable(name) = left {
                return Ok(Statement::Assignment(name, right));
            } else {
                return Err(ParserError::InvalidSyntax);
            }
        }
        // Equals トークンがない場合は式として処理
        self.expect_token(Token::Semicolon)?; // 式の後にセミコロンを期待
        Ok(Statement::Expression(left))
    }

    // 識別子の解析
    fn parse_identifier(&mut self) -> Result<String, ParserError> {
        match self.consume() {
            Some(Token::Identifier(name)) => Ok(name),
            _ => Err(ParserError::InvalidSyntax),
        }
    }

    // パラメータリストの解析
    fn parse_parameters(&mut self) -> Result<Vec<(String, Type)>, ParserError> {
        let mut params = Vec::new();
        // パラメータがない場合すぐに終了
        if let Some(Token::RightParen) = self.peek() {
            return Ok(params);
        }

        loop {
            match self.consume() {
                Some(Token::Identifier(name)) => {
                    self.expect_token(Token::Colon)?;
                    let param_type = self.parse_type()?;
                    params.push((name, param_type));
                }
                Some(found) => {
                    return Err(ParserError::UnexpectedToken {
                        expected: String::from("Identifier"),
                        found: format!("{:?}", found),
                    })
                }
                None => return Err(ParserError::UnexpectedEOF),
            }

            match self.peek() {
                Some(Token::RightParen) => break,
                Some(Token::Comma) => {
                    self.consume();
                }
                Some(found) => {
                    return Err(ParserError::UnexpectedToken {
                        expected: String::from("RightParen or Comma"),
                        found: format!("{:?}", found),
                    })
                }
                None => return Err(ParserError::UnexpectedEOF),
            }
        }

        Ok(params)
    }

    // 型の解析
    fn parse_type(&mut self) -> Result<Type, ParserError> {
        match self.consume() {
            Some(Token::Identifier(type_name)) => match type_name.as_str() {
                "int" => Ok(Type::Int),
                // 他の型に対してもここで処理
                _ => Err(ParserError::UnexpectedToken {
                    expected: String::from("int (or other type)"),
                    found: type_name,
                }),
            },
            Some(found) => Err(ParserError::UnexpectedToken {
                expected: String::from("Identifier"),
                found: format!("{:?}", found),
            }),
            None => Err(ParserError::UnexpectedEOF),
        }
    }

    // ブロック(複数の文の集合)の解析
    pub fn parse_block(&mut self) -> Result<Vec<Statement>, ParserError> {
        let mut statements = Vec::new();
        self.expect_token(Token::LeftBrace)?;

        while let Some(token) = self.peek() {
            if *token == Token::RightBrace {
                break;
            }
            let statement = self.parse_statement()?;

            let is_if_statement = matches!(&statement, Statement::If(..));

            if !is_if_statement && !self.check(&Token::RightBrace) {
                if self.peek() != Some(&Token::Else) && self.peek() != Some(&Token::If) {
                    self.expect_token(Token::Semicolon)?;
                }
            }

            statements.push(statement);
        }

        self.expect_token(Token::RightBrace)?;
        Ok(statements)
    }

    // ブロック内の文の解析
    fn parse_block_contents(&mut self) -> Result<Vec<Statement>, ParserError> {
        let mut statements = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            let statement = self.parse_statement()?;

            // `If` ステートメントの後にはセミコロンを期待しない
            if !matches!(&statement, Statement::If(..)) {
                if !self.check(&Token::RightBrace) && self.peek() != Some(&Token::Else) {
                    self.expect_token(Token::Semicolon)?;
                }
            }

            statements.push(statement);
        }
        Ok(statements)
    }
    // if文の解析
    fn parse_if_statement(&mut self) -> Result<Statement, ParserError> {
        self.expect_token(Token::If)?;
        let condition = self.parse_expression()?;
        self.expect_token(Token::LeftBrace)?;

        let then_branch = self.parse_block_contents()?;
        self.expect_token(Token::RightBrace)?;

        let else_branch = if self.match_token(Token::Else) {
            self.expect_token(Token::LeftBrace)?;
            let else_statements = self.parse_block_contents()?;
            self.expect_token(Token::RightBrace)?;
            Some(Box::new(Statement::Block(else_statements)))
        } else {
            println!("No else branch, current token: {:?}", self.peek());
            None
        };

        Ok(Statement::If(
            Box::new(condition),
            Box::new(Statement::Block(then_branch)),
            else_branch,
        ))
    }
    // 文の解析
    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        println!("Parsing statement, current token: {:?}", self.peek());
        let result = match self.peek() {
            Some(Token::Let) => {
                let stmt = self.parse_declaration()?;
                self.expect_token(Token::Semicolon)?; // 変数宣言の後にセミコロンを期待
                Ok(stmt)
            }
            Some(Token::Fn) => Ok(Statement::Function(self.parse_function()?)),
            Some(Token::If) => {
                let stmt = self.parse_if_statement()?;
                Ok(stmt)
            }
            Some(Token::Print) => {
                let stmt = self.parse_print_statement()?;
                self.expect_token(Token::Semicolon)?; // print文の後にセミコロンを期待
                Ok(stmt)
            }
            Some(Token::Identifier(_)) if self.peek_next() == Some(&Token::Equals) => {
                // Identifier トークンの後に Equals が来た場合（代入文）
                self.parse_assignment_or_expression_statement()
            }
            Some(Token::Identifier(_)) => {
                // Identifier トークンの後に Equals 以外が来た場合（式）
                let expr = self.parse_expression()?;
                self.expect_token(Token::Semicolon)?; // 式の後にセミコロンを期待
                Ok(Statement::Expression(expr))
            }
            _ => Err(ParserError::UnexpectedEOF),
        };
        result
    }
}