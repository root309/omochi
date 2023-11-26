# メモ
## 環境
- Ubuntu 23.04
- [LLVM 12.0.0](https://github.com/llvm/llvm-project/releases?page=6) clang+llvm-12.0.0-x86_64-linux-gnu-ubuntu-20.04.tar.xz

#### LLVM IR からアセンブリ言語へのコンパイル
```
llc -relocation-model=pic -filetype=asm output.ll -o output.s
```

#### アセンブリ言語から実行可能ファイルへのコンパイル
```
gcc -fPIE -pie output.s -o program
```