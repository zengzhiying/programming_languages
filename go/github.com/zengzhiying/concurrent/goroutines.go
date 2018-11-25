package main

import "fmt"
import "time"

func say(s string) {
    for i := 0; i < 5; i++ {
        time.Sleep(100 * time.Millisecond)
        fmt.Println(s)
    }
}

func main() {
    // 轻量级线程
    go say("world")   // go程在相同的地址空间中运行, 同属于1个进程
    say("hello")
}
