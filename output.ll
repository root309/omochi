; ModuleID = 'main'
source_filename = "main"

@fmt = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@fmt.2 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1

define i32 @main() {
entry:
  %w = alloca i32, align 4
  %v = alloca i32, align 4
  %u = alloca i32, align 4
  %t = alloca i32, align 4
  %s = alloca i32, align 4
  %i1 = alloca i32, align 4
  %q = alloca i32, align 4
  %p = alloca i32, align 4
  %o = alloca i32, align 4
  %m = alloca i32, align 4
  %n = alloca i32, align 4
  %l = alloca i32, align 4
  %k = alloca i32, align 4
  %j = alloca i32, align 4
  %i = alloca i32, align 4
  %h = alloca i32, align 4
  %g = alloca i32, align 4
  %f = alloca i32, align 4
  %e = alloca i32, align 4
  %d = alloca i32, align 4
  %c = alloca i32, align 4
  %b = alloca i32, align 4
  %a = alloca i32, align 4
  %z = alloca i32, align 4
  %y = alloca i32, align 4
  %x = alloca i32, align 4
  store i32 6, i32* %x, align 4
  store i32 6, i32* %y, align 4
  store i32 3, i32* %z, align 4
  store i32 4, i32* %a, align 4
  store i32 5, i32* %b, align 4
  store i32 -1486618624, i32* %c, align 4
  store i32 249, i32* %d, align 4
  store i32 9384, i32* %e, align 4
  store i32 488, i32* %f, align 4
  store i32 298374, i32* %g, align 4
  store i32 38274, i32* %h, align 4
  store i32 829374, i32* %i, align 4
  store i32 293847, i32* %j, align 4
  store i32 2390482, i32* %k, align 4
  store i32 209384, i32* %l, align 4
  store i32 23984, i32* %n, align 4
  store i32 293874, i32* %m, align 4
  store i32 98098980, i32* %o, align 4
  store i32 29384, i32* %p, align 4
  store i32 29834, i32* %q, align 4
  store i32 90492, i32* %i1, align 4
  store i32 29384, i32* %s, align 4
  store i32 89898, i32* %t, align 4
  store i32 4444, i32* %u, align 4
  store i32 983, i32* %v, align 4
  store i32 9898, i32* %w, align 4
  %x2 = load i32, i32* %x, align 4
  %gttmp = icmp sgt i32 %x2, 5
  br i1 %gttmp, label %then, label %else

then:                                             ; preds = %entry
  %x3 = load i32, i32* %x, align 4
  %printf_call = call i8* (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt, i32 0, i32 0), i32 %x3)
  br label %ifcont

else:                                             ; preds = %entry
  %x4 = load i32, i32* %x, align 4
  %printf_call5 = call i8* (i8*, ...) @printf.1(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt.2, i32 0, i32 0), i32 %x4)
  br label %ifcont

ifcont:                                           ; preds = %else, %then
  ret i32 0
}

declare i8* @printf(i8*, ...)

declare i8* @printf.1(i8*, ...)
