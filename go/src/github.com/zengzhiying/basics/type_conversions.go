package main

import (
	"fmt"
	"math"
)

func main() {
	var x, y int = 3, 4
    // 类型转换 int -> float64
	var f float64 = math.Sqrt(float64(x*x + y*y))
    // float64 -> uint
	var z uint = uint(f)
	fmt.Println(x, y, z)

    i := 42
    f1 := float64(i)
    u := uint(f1)
    fmt.Println(f1, u)

    // go 必须显示转换, 隐式不会自动转换会报错, 这点和C不一样
    // var f2 float64 = i
    // fmt.Println(f2)

    // go会根据变量赋值的精度自动推导类型
    j := 42  // int
    f3 := 3.1416  // float64
    g := 0.867 + 0.5i  // complex128
    // Printf才可以用占位符, Println不行.
    fmt.Printf("j type: %T, f3 type %T, g type %T\n", j, f3, g)
}
