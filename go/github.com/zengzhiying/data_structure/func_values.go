package main

import (
    "fmt"
    "math"
)

// 函数可以作为数据结构来使用


// 函数可以作为另一个函数的参数 格式: 函数类型参数 func(参数类型列表) 返回值
// 其实就和函数定义类似, 只不过去掉参数名, 只保留参数类型
func compute(fn func(float64, float64) float64) float64 {
    return fn(3, 4)
}

func compute1(x, y float64) float64 {
    return math.Sqrt(x * x + y * y)
}

func main() {
    // 定义变量符号指向闭包函数
    hypot := func(x, y float64) float64 {
        return math.Sqrt(x * x + y * y)
    }
    // 通过变量调用函数
    fmt.Println(hypot(5, 12))

    fmt.Println(compute(hypot))
    fmt.Println(compute(math.Pow))

    fun_call := compute1
    fmt.Println(fun_call(3, 4))
}
