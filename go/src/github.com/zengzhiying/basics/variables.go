package main

import "fmt"

// 使用var关键字定义变量, 类型在最后, 这里变量级别在包
var c, python, java bool

// 声明的时候初始化值
var j, k int = 1, 2

func main() {
    // 局部变量
    var i int

    // 初始化值, 可以不用带类型
    var p, q, r = true, false, "no!"
    fmt.Println(i, c, python, java, j, k, p, q, r)

    // 简洁赋值语句(必须初始化值, 不允许加类型), 可以代替var, 但是只能在函数内使用
    // 函数外所有语句都必须都关键字, 比如: var, func等
    s := 3
    t, u, v := 1, "u", "v"
    fmt.Println(s, t, u, v)
}
