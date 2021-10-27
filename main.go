// ❯ cat main.go

package main

func run() int {
	HOGE := 1
	HUGA := 2
	return HOGE + HUGA
}

// ❯ go tool compile -S -N -l    main.go

// TEXT    "".run(SB), NOSPLIT|ABIInternal, $24-8
// SUBQ    $24, SP
// MOVQ    BP, 16(SP)
// LEAQ    16(SP), BP
// MOVQ    $0, "".~r0+32(SP)
// MOVQ    $1, "".HOGE+8(SP)
// MOVQ    $2, "".HUGA(SP)
// MOVQ    "".HOGE+8(SP), AX
// ADDQ    $2, AX
// MOVQ    AX, "".~r0+32(SP)
// MOVQ    16(SP), BP
// ADDQ    $24, SP
// NOP
// RET
