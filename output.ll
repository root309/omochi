; ModuleID = 'main'
source_filename = "main"

define i32 @main() {
entry:
  %x = alloca i32, align 4
  store i32 5, i32* %x, align 4
}
