package main

import "encoding/hex"

func run(x int) int {
	return x + hex.DecodedLen(100)
}
