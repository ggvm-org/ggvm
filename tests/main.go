package main

import "os"

func add2(i int) int

func main() {
    i := add2(20)
    os.Exit(i)
}