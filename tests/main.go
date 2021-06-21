package main

import "os"

// TODO: 動的に関数名を生成してチェックできるようにする
func add2() int

func main() {
    i := add2()
    os.Exit(i)
}