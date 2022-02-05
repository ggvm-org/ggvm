TEXT    main·run(SB), 4, $32-16
SUBQ    $32, SP
MOVQ    BP, 24(SP)
LEAQ    24(SP), BP
MOVQ    $0, r1+48(SP)
MOVQ    $100, (SP)
PCDATA  $1, $0
CALL    encoding/hex·DecodedLen(SB)
MOVQ    8(SP), AX
MOVQ    AX, autotmp_2+16(SP)
MOVQ    x+40(SP), CX
ADDQ    CX, AX
MOVQ    AX, r1+48(SP)
MOVQ    24(SP), BP
ADDQ    $32, SP
RET
