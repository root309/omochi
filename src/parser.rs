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
    fn check(&self, token: Token) -> bool {
        self.peek().map_or(false, |t| *t == token)
    }
    // 指定したトークンが現在のトークンであれば、それを消費して true を返す
    fn match_token(&mut self, token: Token) -> bool {
        if self.check(token) {
            self.consume();
            true
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
    // 正しく解析されている
    // if文の解析
    fn parse_if_statement(&mut self) -> Result<Statement, ParserError> {
        println!("Parsing if statement, current token: {:?}", self.peek());
        self.expect_token(Token::If)?;
        let condition = self.parse_expression()?;
        self.expect_token(Token::LeftBrace)?;

        println!("Parsing then branch, current token: {:?}", self.peek());
        let then_branch = self.parse_block_contents()?;
        println!(
            "Finished parsing then branch, current token: {:?}",
            self.peek()
        );
        self.expect_token(Token::RightBrace)?;
        println!(
            "Finished parsing if statement, current token: {:?}",
            self.peek()
        );
        let else_branch = if self.match_token(Token::Else) {
            println!("Parsing else branch, current token: {:?}", self.peek());
            self.expect_token(Token::LeftBrace)?;
            let else_statements = self.parse_block_contents()?;
            println!(
                "Finished parsing else branch, current token: {:?}",
                self.peek()
            );
            self.expect_token(Token::RightBrace)?;
            Some(Box::new(Statement::Block(else_statements)))
        } else {
            None
        };
        println!(
            "Finished parsing if statement, current token: {:?}",
            self.peek()
        );
        Ok(Statement::If(
            Box::new(condition),
            Box::new(Statement::Block(then_branch)),
            else_branch,
        ))
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
        let expr = self.parse_expression()?;
        match expr {
            Expr::Assign(name, value) => {
                self.expect_token(Token::Semicolon)?; // 代入文の後にセミコロンを期待
                Ok(Statement::Assignment(name, *value))
            }
            _ => Ok(Statement::Expression(expr)),
        }
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

            println!("Parsing block, current token: {:?}", self.peek());
            let statement = self.parse_statement()?;

            // `If` ステートメントの後にはセミコロンを期待しない
            let is_if_statement = matches!(&statement, Statement::If(..));

            // statement をベクトルに追加する前にセミコロンをチェック
            if !is_if_statement
                && self.peek() != Some(&Token::Else)
                && !self.check(Token::RightBrace)
            {
                println!("Expecting semicolon, current token: {:?}", self.peek());
                self.expect_token(Token::Semicolon)?;
            }

            statements.push(statement);
            println!("Finished parsing block, current token: {:?}", self.peek());
        }

        self.expect_token(Token::RightBrace)?;
        Ok(statements)
    }
    // ブロック内の文の解析
    fn parse_block_contents(&mut self) -> Result<Vec<Statement>, ParserError> {
        let mut statements = Vec::new();
        while !self.check(Token::RightBrace) && !self.is_at_end() {
            let statement = self.parse_statement()?;

            // `If` ステートメントの後にはセミコロンを期待しない
            if !matches!(&statement, Statement::If(..)) {
                if !self.check(Token::RightBrace) && self.peek() != Some(&Token::Else) {
                    self.expect_token(Token::Semicolon)?;
                }
            }

            statements.push(statement);
        }
        Ok(statements)
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
                println!(
                    // ここ呼ばれてないWTF
                    "After parsing if statement, current token: {:?}",
                    self.peek()
                );
                // If ステートメントの後にはセミコロンを期待しない
                Ok(stmt)
            }
            Some(Token::Print) => {
                let stmt = self.parse_print_statement()?;
                self.expect_token(Token::Semicolon)?; // print文の後にセミコロンを期待
                Ok(stmt)
            }
            Some(Token::Identifier(_)) => self.parse_assignment_or_expression_statement(),
            _ => Err(ParserError::UnexpectedEOF),
        };
        println!(
            "Finished parsing statement, current token: {:?}",
            self.peek()
        );
        result
    }

    // カーソルを進める補助関数
    fn advance(&mut self) {
        self.consume();
    }

    // 直前のトークンを取得する補助関数
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
#[cfg(test)]
mod tests {
    use crate::ast::{Expr, Operator, Statement};
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn parse(input: &str) -> Result<Vec<Statement>, String> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex().map_err(|e| e.to_string())?;
        let mut parser = Parser::new(tokens);
        parser.parse_block().map_err(|e| format!("{:?}", e))
    }

    #[test]
    fn test_if_statement() {
        let input = "{ if (x > 5) { x = 10; } else { x = 0; } }";
        let statements = parse(input).expect("Failed to parse if statement");

        assert_eq!(statements.len(), 1);
        match &statements[0] {
            Statement::If(condition, then_branch, else_branch) => {
                match &**condition {
                    Expr::BinaryOp(lhs, op, rhs) => match (&**lhs, op, &**rhs) {
                        (Expr::Variable(name), Operator::MoreThan, Expr::Integer(value)) => {
                            assert_eq!(name, "x");
                            assert_eq!(*value, 5);
                        }
                        _ => panic!("Expected a binary operation"),
                    },
                    _ => panic!("Expected a binary operation in if condition"),
                }

                match &**then_branch {
                    Statement::Block(statements) => {
                        assert_eq!(statements.len(), 1);
                        match &statements[0] {
                            Statement::Assignment(var, expr) => {
                                assert_eq!(var, "x");
                                match expr {
                                    Expr::Integer(value) => assert_eq!(*value, 10),
                                    _ => panic!("Expected an integer expression"),
                                }
                            }
                            _ => panic!("Expected an assignment statement"),
                        }
                    }
                    _ => panic!("Expected a block statement"),
                }

                match else_branch {
                    Some(branch) => match &**branch {
                        Statement::Block(statements) => {
                            assert_eq!(statements.len(), 1);
                            match &statements[0] {
                                Statement::Assignment(var, expr) => {
                                    assert_eq!(var, "x");
                                    match expr {
                                        Expr::Integer(value) => assert_eq!(*value, 0),
                                        _ => panic!("Expected an integer expression"),
                                    }
                                }
                                _ => panic!("Expected an assignment statement"),
                            }
                        }
                        _ => panic!("Expected a block statement"),
                    },
                    None => panic!("Expected an else branch"),
                }
            }
            _ => panic!("Expected an if statement"),
        }
    }
}
