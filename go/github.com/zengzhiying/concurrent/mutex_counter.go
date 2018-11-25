package main

import (
    "fmt"
    "sync"
    "time"
)

// 定义结构体SafeCounter, 并且并发使用是安全的
type SafeCounter struct {
    v map[string] int  // 计数map
    mux sync.Mutex   // 互斥锁
}

// 为结构体绑定计数器方法
func (c *SafeCounter) incr(key string) {
    c.mux.Lock()
    // lock之后同一时刻只能有1个goroutine访问c.v
    // 默认key不存在第一次为0
    // c.v[key]++
    _, ok := c.v[key]
    if ok {
        c.v[key]++
    } else {
        c.v[key] = 1
    }
    c.mux.Unlock()
}

// 为结构体绑定返回统计值的方法
func (c *SafeCounter) getCount(key string) int {
    c.mux.Lock()
    // 因为return之后无法执行解锁语句, 所以使用defer延迟解锁
    defer c.mux.Unlock()    // 保证互斥锁一定会被解锁
    return c.v[key]
}

func main() {
    c := SafeCounter{v: make(map[string] int)}
    for i := 0; i < 1000; i++ {
        go c.incr("somekey")
    }
    time.Sleep(time.Second)
    fmt.Println(c.getCount("somekey"))
}
