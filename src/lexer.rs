// use regex::Regex;
use std::str::Chars;

// トークンの種類を定義する列挙型
#[derive(Debug, PartialEq)]
enum Token {
    Let,               // `let` キーワード
    Identifier(String),// 識別子
    Integer(i64),      // 整数リテラル
    Plus,              // 加算演算子
    Equals,            // 等号
    EOF,               // 入力の終了
}

// 字句解析器の構造体
struct Lexer<'a> {
    input: Chars<'a>,         // 入力文字列
    current_char: Option<char>,// 現在の文字
}

impl<'a> Lexer<'a> {
    // 新しい字句解析器のインスタンスを作成
    fn new(input: &'a str) -> Lexer<'a> {
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

    // 字句解析のメインロジック
    fn lex(&mut self) -> Vec<Token> {
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
                // 等号
                '=' => {
                    tokens.push(Token::Equals);
                    self.next_char();
                }
                // 数字なら整数リテラルの解析を行う
                '0'..='9' => tokens.push(self.integer()),
                // 英字またはアンダースコアなら識別子の解析を行う
                'a'..='z' | 'A'..='Z' | '_' => tokens.push(self.identifier()),
                // それ以外の文字は未実装
                _ => unimplemented!("エラーハンドリングが必要"),
            }
        }
        tokens.push(Token::EOF); // 入力の終了を表すトークンを追加
        tokens
    }

    // 整数リテラルを解析する関数
    fn integer(&mut self) -> Token {
        let mut number = String::new();
        while let Some('0'..='9') = self.current_char {
            number.push(self.current_char.unwrap());
            self.next_char();
        }
        Token::Integer(number.parse::<i64>().unwrap())
    }

    // 識別子を解析する関数
    fn identifier(&mut self) -> Token {
        let mut identifier = String::new();
        while let Some('a'..='z' | 'A'..='Z' | '0'..='9' | '_') = self.current_char {
            identifier.push(self.current_char.unwrap());
            self.next_char();
        }
        match identifier.as_str() {
            // 予約語 `let`
            "let" => Token::Let,
            // それ以外は一般の識別子
            _ => Token::Identifier(identifier),
        }
    }
}

// 字句解析器のテスト
#[test]
fn test_lexer() {
    let mut lexer = Lexer::new("let x = 5 + 10");
    let tokens = lexer.lex();
    assert_eq!(
        tokens,
        vec![
            Token::Let,
            Token::Identifier("x".to_string()),
            Token::Equals,
            Token::Integer(5),
            Token::Plus,
            Token::Integer(10),
            Token::EOF
        ]
    );
}