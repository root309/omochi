# IR生成
echo "Generating LLVM IR..."
cargo run

# LLVM IRからアセンブリ言語へのコンパイル
echo "Compiling LLVM IR to assembly..."
llc -relocation-model=pic -filetype=asm output.ll -o output.s

# アセンブリ言語から実行可能ファイルへのコンパイル
echo "Compiling assembly to executable..."
gcc -fPIE -pie output.s -o program

echo "Build complete."
