#include "go_asm.h"

TEXT    main·run(SB), NOSPLIT, $0-8
SUBQ    $16, SP
MOVQ    BP, 8(SP)
LEAQ    8(SP), BP
CALL    s(SB)
LEAQ    r0+8(SP), AX
MOVQ    $100, r0+24(SP)
MOVQ    8(SP), BP
ADDQ    $16, SP
RET

TEXT    s(SB), NOSPLIT, $0-8
MOVQ    $1, r0+8(SP)
RET
