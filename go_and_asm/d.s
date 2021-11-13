TEXT	main·run(SB), 4, $16-8
SUBQ	$16, SP
MOVQ	BP, 8(SP)
LEAQ	8(SP), BP
FUNCDATA	$0, gclocals·33cdeccccebe80329f1fdbee7f5874cb(SB)
FUNCDATA	$1, gclocals·33cdeccccebe80329f1fdbee7f5874cb(SB)
// 本質はこの下

MOVQ	$0, r0+24(SP) // r0+24(SP) = 返り値のメモリ番地を `0` で初期化
MOVQ	$0, thisIsVar(SP) // var thisIsVar int、thisIsVarのメモリを `0` で初期化
MOVQ	$100, thisIsVar(SP) // thisIsVar = 100
MOVQ	$100, r0+24(SP) // なんで最適化切ってるのに定数畳み込みするの😡😡😡😡

// ここまで本質
MOVQ	8(SP), BP
ADDQ	$16, SP
RET



