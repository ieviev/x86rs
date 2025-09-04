	.file	"main.5b14f2d4376fcf55-cgu.0"
	.section	.text._start,"ax",@progbits
	.globl	_start
	.p2align	4
	.type	_start,@function
_start:
.Lfunc_begin0:
	.cfi_sections .debug_frame
	.cfi_startproc
	pushq	%rbx
	.cfi_def_cfa_offset 16
	subq	$4096, %rsp
	.cfi_def_cfa_offset 4112
	.cfi_offset %rbx, -16
	movl	$4096, %edx
	movq	%rsp, %rbx
	movq	%rbx, %rdi
	xorl	%esi, %esi
	callq	*memset@GOTPCREL(%rip)
	movl	$4096, %edx
	xorl	%edi, %edi
	movq	%rbx, %rsi
	xorl	%eax, %eax
	#APP

	syscall

	#NO_APP
	movl	$1, %edi
	leaq	.Lanon.aba89868ca648563d3cb6501b53f6d25.0(%rip), %rsi
	movl	$13, %edx
	movl	$1, %eax
	#APP

	syscall

	#NO_APP
	movl	$60, %eax
	xorl	%edi, %edi
	#APP

	syscall

	#NO_APP
	ud2
.Lfunc_end0:
	.size	_start, .Lfunc_end0-_start
	.cfi_endproc

	.type	.Lanon.aba89868ca648563d3cb6501b53f6d25.0,@object
	.section	.rodata..Lanon.aba89868ca648563d3cb6501b53f6d25.0,"a",@progbits
.Lanon.aba89868ca648563d3cb6501b53f6d25.0:
	.ascii	"hello world!\n"
	.size	.Lanon.aba89868ca648563d3cb6501b53f6d25.0, 13

	.ident	"rustc version 1.91.0-nightly (51ff89506 2025-09-02)"
	.ident	"rustc version 1.91.0-nightly (51ff89506 2025-09-02)"
	.section	".note.GNU-stack","",@progbits
