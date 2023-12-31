use crate::ast::Token;
use std::fmt;
use std::str::Chars;

// 字句解析器のエラーを表す列挙型
#[derive(Debug)]
pub enum LexerError {
    UnknownToken(char),
    InvalidNumber(String),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::UnknownToken(c) => write!(f, "Unknown token: {}", c),
            LexerError::InvalidNumber(n) => write!(f, "Invalid number: {}", n),
        }
    }
}
// 字句解析器本体の構造体
pub struct Lexer<'a> {
    input: Chars<'a>,           // 入力文字列
    current_char: Option<char>, // 現在解析中の文字
}

impl<'a> Lexer<'a> {
    // 新しい字句解析器のインスタンスを作成
    pub fn new(input: &'a str) -> Lexer<'a> {
        let mut lexer = Lexer {
            input: input.chars(),
            current_char: None,
        };
        lexer.next_char(); // 最初の文字を読み込む
        lexer
    }

    // 次の文字に進む
    fn next_char(&mut self) {
        self.current_char = self.input.next();
    }
    // 現在の文字と次の文字をチェックする
    fn peek_next_char(&self) -> Option<char> {
        self.input.clone().next()
    }
    // 字句解析のメインロジック
    pub fn lex(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        while let Some(ch) = self.current_char {
            match ch {
                // 空白文字は無視
                ' ' | '\n' | '\t' | '\r' => self.next_char(),
                // 加算演算子
                '+' => {
                    tokens.push(Token::Plus);
                    self.next_char();
                }
                // 減算演算子,アロー演算子
                '-' => {
                    // 次の文字が '>' なら Arrow トークンを作成
                    if self.peek_next_char() == Some('>') {
                        tokens.push(Token::Arrow);
                        self.next_char(); // '>' を消費
                    } else {
                        tokens.push(Token::Minus);
                    }
                    self.next_char();
                }
                // 等号,ダブルイコール
                '=' => {
                    if let Some('=') = self.peek_next_char() {
                        self.next_char(); // '=' をスキップ
                        self.next_char(); // '=' をスキップ
                        tokens.push(Token::DoubleEquals);
                    } else {
                        tokens.push(Token::Equals);
                        self.next_char();
                    }
                }
                // 左括弧
                '(' => {
                    tokens.push(Token::LeftParen);
                    self.next_char();
                }
                // 右括弧
                ')' => {
                    tokens.push(Token::RightParen);
                    self.next_char();
                }
                // 左中括弧
                '{' => {
                    tokens.push(Token::LeftBrace);
                    self.next_char();
                }
                // 右中括弧
                '}' => {
                    tokens.push(Token::RightBrace);
                    self.next_char();
                }
                // セミコロン
                ';' => {
                    tokens.push(Token::Semicolon); // セミコロンの解析
                    self.next_char();
                }
                // 大なり演算子
                '>' => {
                    tokens.push(Token::MoreThan);
                    self.next_char();
                }
                // 小なり演算子
                '<' => {
                    tokens.push(Token::LessThan);
                    self.next_char();
                }
                // アスタリスク（乗算演算子）
                '*' => {
                    tokens.push(Token::Asterisk);
                    self.next_char();
                }
                // コロン
                ':' => {
                    tokens.push(Token::Colon);
                    self.next_char();
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.next_char();
                }
                // 数字なら整数リテラルの解析を行う
                '0'..='9' => tokens.push(self.integer()?),
                // 英字またはアンダースコアなら識別子の解析を行う
                'a'..='z' | 'A'..='Z' | '_' => tokens.push(self.identifier()?),
                // 例外
                _ => return Err(LexerError::UnknownToken(ch)),
            }
        }
        tokens.push(Token::EOF); // 入力の終了を表すトークン
        Ok(tokens)
    }

    // 整数リテラルを解析する関数
    fn integer(&mut self) -> Result<Token, LexerError> {
        let mut number = String::new();
        while let Some('0'..='9') = self.current_char {
            number.push(self.current_char.unwrap());
            self.next_char();
        }
        number
            .parse::<i64>()
            .map(Token::Integer)
            .map_err(|_| LexerError::InvalidNumber(number))
    }

    // 識別子を解析する関数
    fn identifier(&mut self) -> Result<Token, LexerError> {
        let mut identifier = String::new();
        while let Some('a'..='z' | 'A'..='Z' | '0'..='9' | '_') = self.current_char {
            identifier.push(self.current_char.unwrap());
            self.next_char();
        }
        Ok(match identifier.as_str() {
            "let" => Token::Let,
            "fn" => Token::Fn,
            "if" => Token::If,
            "print" => Token::Print,
            _ => Token::Identifier(identifier),
        })
    }
}
