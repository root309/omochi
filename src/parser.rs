use crate::ast::{Expr, Operator, Token};

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken,
    //UnexpectedEOF,
}
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse_expression(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.parse_primary()?;

        while let Some(op) = self.next_operator()? {
            let right = self.parse_primary()?;
            expr = Expr::BinaryOp(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, ParserError> {
        match self.tokens.get(self.current) {
            Some(Token::Integer(value)) => {
                self.current += 1;
                Ok(Expr::Integer(*value))
            }
            _ => Err(ParserError::UnexpectedToken),
        }
    }

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
    // TODO:thenの実装に使う
    // fn match_tokens(&mut self, types: &[Token]) -> bool {
    //     for token_type in types {
    //         if self.check(token_type) {
    //             self.advance();
    //             return true;
    //         }
    //     }
    //     false
    // }
    // TODO:三項演算子やらを実装するのに使う
    // fn check(&self, token_type: &Token) -> bool {
    //     if self.is_at_end() {
    //         return false;
    //     }
    //     self.tokens.get(self.current).unwrap() == token_type
    // }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.tokens.get(self.current), Some(Token::EOF) | None)
    }

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