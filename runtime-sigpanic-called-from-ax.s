#include "go_asm.h"

TEXT main·run(SB), 4, $0
SUBQ $16, SP
MOVQ BP, 8(SP)
LEAQ 8(SP), BP

MOVQ $11, AX

MOVQ AX, ret+8(FP)
MOVQ 8(SP), BP
ADDQ $16, SP
RET

