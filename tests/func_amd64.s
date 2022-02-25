TEXT	main·run(SB), 1|4, $8-0
CMPQ	SP, 16(R14)
PCDATA	$0, $-2
JLS	epi

body:
PCDATA	$0, $-1
SUBQ	$8, SP
MOVQ	BP, (SP)
LEAQ	(SP), BP
FUNCDATA	$0, gclocals·33cdeccccebe80329f1fdbee7f5874cb(SB)
FUNCDATA	$1, gclocals·33cdeccccebe80329f1fdbee7f5874cb(SB)
PCDATA	$1, $0
CALL	main·p(SB)
MOVQ	(SP), BP
ADDQ	$8, SP
NOP
RET

epi:
    NOP
    PCDATA	$1, $-1
    PCDATA	$0, $-2
    CALL	runtime·morestack_noctxt(SB)
    PCDATA	$0, $-1
    JMP	body

TEXT	main·c(SB), 1|4, $0-0
FUNCDATA	$0, gclocals·33cdeccccebe80329f1fdbee7f5874cb(SB)
FUNCDATA	$1, gclocals·33cdeccccebe80329f1fdbee7f5874cb(SB)
RET
