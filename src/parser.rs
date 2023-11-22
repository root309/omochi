use crate::ast::{Expr, Operator, Token, Statement, Function, Type};

// 構文解析器のエラーを表す列挙型
#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken,
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
    // 次のトークンを消費して、カーソルを進める
    fn consume(&mut self) -> Option<Token> {
        if self.is_at_end() {
            None
        } else {
            let token = self.tokens[self.current].clone();
            self.current += 1;
            Some(token)
        }
    }

    // 現在のトークンを取得(消費しない)
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    // 指定されたトークンを期待しているか確認し、そうでなければエラー
    fn expect_token(&mut self, expected: Token) -> Result<(), ParserError> {
        let token = self.consume().ok_or(ParserError::UnexpectedEOF)?;
        if token == expected {
            Ok(())
        } else {
            Err(ParserError::UnexpectedToken)
        }
    }

    // 現在のトークンがEOF(入力終了)かどうか
    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Some(Token::EOF) | None)
    }

    // 式の解析
    pub fn parse_expression(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.parse_primary()?;

        while let Some(op) = self.next_operator()? {
            let right = self.parse_primary()?;
            expr = Expr::BinaryOp(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    // 単項式(数字など基本的な要素)の解析
    fn parse_primary(&mut self) -> Result<Expr, ParserError> {
        match self.tokens.get(self.current) {
            Some(Token::Integer(value)) => {
                self.current += 1;
                Ok(Expr::Integer(*value))
            }
            _ => Err(ParserError::UnexpectedToken),
        }
    }

    // 次の演算子を取得
    fn next_operator(&mut self) -> Result<Option<Operator>, ParserError> {
        let operator = match self.tokens.get(self.current) {
            Some(Token::Plus) => {
                self.advance();
                Some(Operator::Plus)
            },
            Some(Token::Minus) => {
                self.advance();
                Some(Operator::Minus)
            },
            _ => None,
        };
        Ok(operator)
    }

    // 変数宣言の解析
    fn parse_declaration(&mut self) -> Result<Statement, ParserError> {
        self.expect_token(Token::Let)?;
        let name = match self.consume() {
            Some(Token::Identifier(name)) => name,
            _ => return Err(ParserError::UnexpectedToken),
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
        Ok(Function { name, params, return_type, body })
    }

    // if文の解析
    fn parse_if_statement(&mut self) -> Result<Statement, ParserError> {
        self.expect_token(Token::If)?;
        let condition = self.parse_expression()?;
        let then_branch = self.parse_block()?;
        let else_branch = if let Some(Token::Else) = self.peek() {
            self.consume();
            Some(Box::new(Statement::Block(self.parse_block()?)))
        } else {
            None
        };
        Ok(Statement::If(Box::new(condition), Box::new(Statement::Block(then_branch)), else_branch))
    }

    // print文の解析
    fn parse_print_statement(&mut self) -> Result<Statement, ParserError> {
        self.expect_token(Token::Print)?;
        let expr = self.parse_expression()?;
        Ok(Statement::Print(expr))
    }

    // 代入または式の文の解析
    fn parse_assignment_or_expression_statement(&mut self) -> Result<Statement, ParserError> {
        let expr = self.parse_expression()?;
        if let Expr::Assign(name, value) = expr {
            Ok(Statement::Assignment(name, *value))
        } else {
            Ok(Statement::Expression(expr))
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
            if let Some(Token::Identifier(name)) = self.consume() {
                self.expect_token(Token::Colon)?;
                let param_type = self.parse_type()?;
                params.push((name, param_type));
            } else {
                return Err(ParserError::UnexpectedToken);
            }
    
            match self.peek() {
                Some(Token::RightParen) => break,
                Some(Token::Comma) => { self.consume(); },
                _ => return Err(ParserError::UnexpectedToken),
            }
        }
    
        Ok(params)
    }

    // TODO:型推論作りたい
    // 型の解析
    fn parse_type(&mut self) -> Result<Type, ParserError> {
        match self.consume() {
            Some(Token::Identifier(type_name)) => match type_name.as_str() {
                "int" => Ok(Type::Int),
                // 他の型に対してもここで処理
                _ => Err(ParserError::UnexpectedToken),
            },
            _ => Err(ParserError::UnexpectedToken),
        }
    }

    // ブロック(複数の文の集合)の解析
    fn parse_block(&mut self) -> Result<Vec<Statement>, ParserError> {
        let mut statements = Vec::new();
        self.expect_token(Token::LeftBrace)?;
    
        while let Some(token) = self.peek() {
            if *token == Token::RightBrace {
                break;
            }
            let statement = self.parse_statement()?;
            statements.push(statement);
        }
    
        self.expect_token(Token::RightBrace)?;
        Ok(statements)
    }

    // 文の解析
    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        let statement = match self.peek() {
            Some(Token::Let) => {
                let stmt = self.parse_declaration()?;
                self.expect_token(Token::Semicolon)?; // 変数宣言の後にセミコロンを期待
                stmt
            },
            Some(Token::Fn) => {
                let function = self.parse_function()?;
                Statement::Function(function) // 関数宣言の後にセミコロンは必要ない
            },
            Some(Token::If) => {
                let stmt = self.parse_if_statement()?;
                self.expect_token(Token::Semicolon)?; // if文の後にセミコロンを期待
                stmt
            },
            Some(Token::Print) => {
                let stmt = self.parse_print_statement()?;
                self.expect_token(Token::Semicolon)?; // print文の後にセミコロンを期待
                stmt
            },
            Some(Token::Identifier(_)) => {
                let stmt = self.parse_assignment_or_expression_statement()?;
                self.expect_token(Token::Semicolon)?; // 代入文または式文の後にセミコロンを期待
                stmt
            },
            _ => return Err(ParserError::InvalidSyntax),
        };
        Ok(statement)
    }

    // カーソルを進める補助関数
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    // 直前のトークンを取得する補助関数
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
#[test]
fn test_parse_expression() {
    let mut parser = Parser::new(vec![
        Token::Integer(3),
        Token::Plus,
        Token::Integer(4),
        Token::Minus,
        Token::Integer(5),
        Token::EOF,
    ]);
    let ast = parser.parse_expression().expect("Failed to parse expression");
    assert_eq!(
        ast,
        Expr::BinaryOp(
            Box::new(Expr::BinaryOp(
                Box::new(Expr::Integer(3)),
                Operator::Plus,
                Box::new(Expr::Integer(4))
            )),
            Operator::Minus,
            Box::new(Expr::Integer(5))
        )
    );
}