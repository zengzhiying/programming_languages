package main

import (
    "fmt"
    "math"
)

type Vertex struct {
    X, Y float64
}

// 为结构体定义方法
func (v Vertex) Abs() float64 {
    return math.Sqrt(v.X * v.X + v.Y * v.Y)
}

// 方法其实就是函数
// 这个函数和上面方法是等效的, 只不过调用方式不太一样而已
func Abs(v Vertex) float64 {
    return math.Sqrt(v.X * v.X + v.Y * v.Y)
}

// 为结构体定义指针接收方法
// 使用指针作为函数参数, 不存在值的拷贝, 对比较大的数据结构更加高效
func (v *Vertex) Scale(f float64) {
    v.X = v.X * f
    v.Y = v.Y * f
}

func main() {
    v := Vertex{3, 4}
    // 调用结构体方法
    fmt.Println(v.Abs())
    // 调用普通函数
    fmt.Println(Abs(v))
    // 调用指针方法修改内部值
    v.Scale(10)
    fmt.Println(v.Abs())
    
    p := &v
    // 这样通过地址调用方法仍然可以, Go会自动解析并转换
    fmt.Println(p.Abs())
}
