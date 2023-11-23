; ModuleID = 'main'
source_filename = "main"

@fmt = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1

define i32 @main() {
entry:
  %y = alloca i32, align 4
  %x = alloca i32, align 4
  store i32 5, i32* %x, align 4
  %x1 = load i32, i32* %x, align 4
  %addtmp = add i32 %x1, 3
  store i32 %addtmp, i32* %y, align 4
  %y2 = load i32, i32* %y, align 4
  %printf_call = call i8* (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt, i32 0, i32 0), i32 %y2)
}

declare i8* @printf(i8*, ...)
