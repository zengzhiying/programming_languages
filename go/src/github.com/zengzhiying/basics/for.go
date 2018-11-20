package main

import "fmt"

func main() {
    // 一般循环
    sum := 0
    for i := 0; i < 10; i++ {
        sum += i
    }
    fmt.Println(sum)
    // 可选初始化和后置语句
    sum = 1
    for ; sum < 1000; {
        sum += sum
    }
    fmt.Println(sum)
    // while语句同样用for表示
    sum = 1
    for sum < 1000 {
        sum += sum
    }
    fmt.Println(sum)
    // 无限循环
    // for {
    //     // code.
    // }
}
