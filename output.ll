; ModuleID = 'main'
source_filename = "main"

@fmt = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1

define i32 @main() {
entry:
  %x = alloca i32, align 4
  store i32 5, i32* %x, align 4
  %x1 = load i32, i32* %x, align 4
  %gttmp = icmp sgt i32 %x1, 5
  br i1 %gttmp, label %then, label %else

then:                                             ; preds = %entry
  store i32 10, i32* %x, align 4
  br label %ifcont

else:                                             ; preds = %entry
  store i32 0, i32* %x, align 4
  br label %ifcont

ifcont:                                           ; preds = %else, %then
  %x2 = load i32, i32* %x, align 4
  %printf_call = call i8* (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt, i32 0, i32 0), i32 %x2)
  ret i32 0
}

declare i8* @printf(i8*, ...)
