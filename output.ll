; ModuleID = 'main'
source_filename = "main"

@fmt = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1

define i32 @main() {
entry:
  %y = alloca i32, align 4
  %x = alloca i32, align 4
  store i32 6, i32* %x, align 4
  store i32 6, i32* %y, align 4
}

define i32 @add() {
entry:
  %sum = alloca i32, align 4
  %x = load i32, i32* %x, align 4
  %y = load i32, i32* %y, align 4
  %addtmp = add i32 %x, %y
  store i32 %addtmp, i32* %sum, align 4
  ret i32 0
  %sum1 = load i32, i32* %sum, align 4
  %printf_call = call i8* (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt, i32 0, i32 0), i32 %sum1)
  ret i32 0
}

declare i8* @printf(i8*, ...)
