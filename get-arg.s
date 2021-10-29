#include "go_asm.h"

// func run(x int) int { return x + 100 }
TEXT main·run(SB),4,$0-8
  MOVQ addr+0(FP), AX
  ADDQ $100, AX
  MOVQ AX, ret+8(FP)
  RET
