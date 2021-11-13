package main

import "os"

// TODO: 動的に関数名を生成してチェックできるようにする
func run(x int) int

func main() {
	i := run(1)
	println(i)
	os.Exit(i)
}
