	.text
	.file	"main"
	.globl	main                            # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:                                # %entry
	subq	$104, %rsp
	.cfi_def_cfa_offset 112
	movl	$6, (%rsp)
	movl	$6, 4(%rsp)
	movl	$3, 8(%rsp)
	movl	$4, 12(%rsp)
	movl	$5, 16(%rsp)
	movl	$-1486618624, 20(%rsp)          # imm = 0xA7640000
	movl	$249, 24(%rsp)
	movl	$9384, 28(%rsp)                 # imm = 0x24A8
	movl	$488, 32(%rsp)                  # imm = 0x1E8
	movl	$298374, 36(%rsp)               # imm = 0x48D86
	movl	$38274, 40(%rsp)                # imm = 0x9582
	movl	$829374, 44(%rsp)               # imm = 0xCA7BE
	movl	$293847, 48(%rsp)               # imm = 0x47BD7
	movl	$2390482, 52(%rsp)              # imm = 0x2479D2
	movl	$209384, 56(%rsp)               # imm = 0x331E8
	movl	$23984, 60(%rsp)                # imm = 0x5DB0
	movl	$293874, 64(%rsp)               # imm = 0x47BF2
	movl	$98098980, 68(%rsp)             # imm = 0x5D8DF24
	movl	$29384, 72(%rsp)                # imm = 0x72C8
	movl	$29834, 76(%rsp)                # imm = 0x748A
	movl	$90492, 80(%rsp)                # imm = 0x1617C
	movl	$29384, 84(%rsp)                # imm = 0x72C8
	movl	$89898, 88(%rsp)                # imm = 0x15F2A
	movl	$4444, 92(%rsp)                 # imm = 0x115C
	movl	$983, 96(%rsp)                  # imm = 0x3D7
	movl	$9898, 100(%rsp)                # imm = 0x26AA
	cmpl	$6, (%rsp)
	jl	.LBB0_2
# %bb.1:                                # %then
	movl	(%rsp), %esi
	leaq	.Lfmt(%rip), %rdi
	xorl	%eax, %eax
	callq	printf@PLT
	jmp	.LBB0_3
.LBB0_2:                                # %else
	movl	(%rsp), %esi
	leaq	.Lfmt.2(%rip), %rdi
	xorl	%eax, %eax
	callq	printf.1@PLT
.LBB0_3:                                # %ifcont
	xorl	%eax, %eax
	addq	$104, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end0:
	.size	main, .Lfunc_end0-main
	.cfi_endproc
                                        # -- End function
	.type	.Lfmt,@object                   # @fmt
	.section	.rodata.str1.1,"aMS",@progbits,1
.Lfmt:
	.asciz	"%d\n"
	.size	.Lfmt, 4

	.type	.Lfmt.2,@object                 # @fmt.2
.Lfmt.2:
	.asciz	"%d\n"
	.size	.Lfmt.2, 4

	.section	".note.GNU-stack","",@progbits
