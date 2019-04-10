package main

import "fmt"

import "cgo_demo/include"

func main() {
    a := 4
    b := 5
    s := include.GoSum(a, b)
    fmt.Printf("%T, %d\n", s, s)
}
