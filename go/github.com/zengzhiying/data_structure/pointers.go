package main

import "fmt"

func main() {
    // 指针初始化, 默认为nil
    var p1 *int
    i, j := 42, 2701

    // 引用
    p1 = &i
    // 初始化并引用
    p2 := &i
    // 取值
    fmt.Println(*p1, *p2)

    // 间接修改内存
    *p2 = 50
    fmt.Println(*p1, i)

    p3 := &j;
    *p3 = *p3 / 37
    fmt.Println(j)
    // 不和C一样, Go没有指针运算, 比如p++等操作
}
