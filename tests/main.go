package main

import (
	"math/rand"

	"github.com/ggvm-org/pure"
)

func run()

func q() {
	pure.Unpure()
}

func LinkP() {
	println(rand.Intn(1000))
}

func p() {
	LinkP()
}

func fib(n int) int {
	return n
}

func drop(x int) {}

func main() {
	run()
	p()
}
