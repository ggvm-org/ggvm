package main

import "os"

// TODO: 動的に関数名を生成してチェックできるようにする
func run() int

func main() {
	i := run()
	println(i)
	os.Exit(i)
}
