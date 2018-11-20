package main

import (
    "fmt"
    "runtime"
    "time"
)

func main() {
    fmt.Print("Go runs on ")
    switch os := runtime.GOOS; os {
    case "darwin":
        fmt.Println("OS X.")
    case "linux":
        fmt.Println("Linux.")
        // 下面语句可以不自动终止case
        // fallthrough
    default:
        fmt.Printf("%s.", os)
    }

    today := time.Now().Weekday()
    // fmt.Println(today)
    // 当前星期几和星期六的距离
    fmt.Print("星期六 ")
    switch time.Saturday {
    case today + 0:
        fmt.Println("今天")
    case today + 1:
        fmt.Println("明天")
    case today + 2:
        fmt.Println("后天")
    default:
        fmt.Println("太远了")
    }

    // 没有条件的switch, 默认为true, 相当于一堆if else的简化
    t := time.Now()
    switch {
    case t.Hour() < 12:
        fmt.Println("上午好！")
    case t.Hour() < 17:
        fmt.Println("下午好！")
    default:
        fmt.Println("晚上好！")
    }
}
