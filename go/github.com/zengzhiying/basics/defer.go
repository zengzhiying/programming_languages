package main

import "fmt"

func main() {
    // 推迟函数, 参数会立即求值, 但是函数会推迟到当前代码外层函数返回之后执行
    defer fmt.Println("world")

    fmt.Println("hello")

    // 推迟函数栈, 压入栈中推迟执行, 当函数返回时会按照后进先出调用
    for i := 0; i < 10; i++ {
        defer fmt.Print(fmt.Sprint(i) + " ")
    }
    fmt.Println("done.")
}
