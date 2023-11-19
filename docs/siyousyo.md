# プロジェクト設計書

## 概要

Rustで開発する関数型プログラミング言語の設計についての説明。この言語は、基本的なプログラミング構造（変数宣言、関数、算術演算、等値比較、条件文）をサポートし、型推論機能を備えている。


## 設計

### 構文

#### 変数宣言

- 形式: `let 変数名 = 式;`
- 例: `let x = 5;`

#### 関数定義

- 形式: `fn 関数名(引数: 型, ...) -> 戻り値の型 { 処理 }`
- 例: `fn add(x: int, y: int) -> int { x + y }`

#### 加算と減算

- 形式: `式 + 式`, `式 - 式`
- 例: `x + y`, `x - y`

#### 等値比較

- 形式: `式 == 式`
- 例: `x == y`

#### 代入文

- 形式: `変数名 = 式;`
- 例: `x = 10;`

#### if 文

- 形式: `if 条件式 then 文;`
- 例: `if x == y then print x;`

#### print 文

- 形式: `print 式;`
- 例: `print x;`

### 型推論

この言語では、変数や関数の型はコードの文脈に基づいて自動的に推論される。ユーザーは型を明示的に宣言する必要がない。

## 実装計画

### 字句解析器（Lexer）

- 入力されたソースコードをトークンに分割する。

### 構文解析器（Parser）

- トークンから抽象構文木（AST）を生成する。

### LLVM IR
ASTをもとにLLVM IRを生成

### 意味解析（Semantic Analysis）

- ASTを解析し、型チェックや変数のスコープを解析する。

### コード生成

- LLVMを使用して最適化された機械語コードを生成する。

# 目標
- 意味解析
- 中間表現 IR生成
- コード生成