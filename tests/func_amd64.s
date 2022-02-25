TEXT	main·run(SB), 4, $32-8
SUBQ	$32, SP
MOVQ	BP, 24(SP)

LEAQ	24(SP), BP
MOVQ	AX, main·x+40(SP)
MOVQ	$10, main·r1+8(SP)
MOVQ	main·x+40(SP), AX
PCDATA	$1, $0
CALL	main·fib(SB)
MOVQ	AX, 16(SP)
MOVQ	AX, main·r1+8(SP)

MOVQ	24(SP), BP
ADDQ	$32, SP
NOP
RET
