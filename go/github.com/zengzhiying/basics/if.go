package main

import (
    "fmt"
    "math"
)

func sqrt(x float64) string {
    // if普通用法
    if x < 0 {
        return sqrt(-x) + "i"
    }
    return fmt.Sprint(math.Sqrt(x))
}

func pow(x, n, lim float64) float64 {
    // if简短语句, 在表达式之前可以执行1个语句
    if v := math.Pow(x, n); v < lim {
        return v
    } else {
        // 这里还可以使用变量v
        fmt.Printf("%g >= %g\n", v, lim)
    }
    // 出来if else就不能使用v了.
    return lim
}

func main() {
    fmt.Println(sqrt(2), sqrt(-4))

    fmt.Println(
        pow(3, 2, 10),
        pow(3, 3, 20),
    )
}
