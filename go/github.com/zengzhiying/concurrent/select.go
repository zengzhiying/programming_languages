package main

import "fmt"
import "time"

func fibonacci(c, quit chan int) {
    x, y := 0, 1
    for {
        // select语句可以使go程等待多个通信操作
        // 其中1个分支未阻塞会执行未阻塞的
        // 所有分支都阻塞会等待执行
        // 所有分支都准备好时会随机选择1个执行
        select {
        // 发送操作
        case c <- x:
            x, y = y, x + y
        // 接收操作
        case <- quit:
            fmt.Println("quit")
            return
        }
    }
}

func select_default() {
    tick := time.Tick(100 * time.Millisecond)
    boom := time.After(500 * time.Millisecond)
    for {
        select {
        case <- tick:
            fmt.Println("tick.")
        case <- boom:
            fmt.Println("BOOM!")
            return
        // default语句会在所有分支都没准备好时执行
        // 用于在发送或者接收时不发生阻塞使用
        default:
            fmt.Println("default.")
            time.Sleep(50 * time.Millisecond)
        }
    }
}


func main() {
    c := make(chan int)
    quit := make(chan int)
    go func() {
        for i := 0; i < 10; i++ {
            fmt.Println(<- c)
        }
        quit <- 0
    }()
    fibonacci(c, quit)
    select_default()
}
