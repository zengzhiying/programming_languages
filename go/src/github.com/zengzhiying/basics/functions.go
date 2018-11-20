package main

import "fmt"

func add(x int, y int) int {
    return x + y
}

// 当函数形参类型相同时, 除了最后一个, 其他类型可以省略
// 即: x int, y int, 可以改为: x, y int
func add1(x, y int) int {
    return x + y
}

// 多值返回, 可以返回任意数量的值
func swap(x, y string) (string, string) {
    return y, x
}

// 命名返回值, 返回定义在函数顶部的变量
// return 没有参数返回已命名的值, 
// 只限用在短函数中, 长的函数影响代码可读性
func split(sum int) (x, y int) {
    x = sum * 4 / 9
    y = sum - x
    return
}

func main() {
    fmt.Println(add(42, 13))
    a, b := swap("hello", "world")
    fmt.Println(a, b)

    fmt.Println(split(17))
}
