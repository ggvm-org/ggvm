TEXT	main·run(SB), 4, $16-16
SUBQ	$16, SP
MOVQ	BP, 8(SP)
LEAQ	8(SP), BP
FUNCDATA	$0, gclocals·33cdeccccebe80329f1fdbee7f5874cb(SB)
FUNCDATA	$1, gclocals·33cdeccccebe80329f1fdbee7f5874cb(SB)
MOVQ	$0, r1+32(SP) // Initialize the Zero value of Go for virtual return variable(r1).
MOVQ	$100, y(SP)
MOVQ	x+24(SP), AX
ADDQ	$100, AX
MOVQ	AX, r1+32(SP)
MOVQ	8(SP), BP
ADDQ	$16, SP
RET
