package main

import "fmt"
import "time"

func sum(s []int, c chan int) {
    sum := 0
    for _, v := range s {
        sum += v
    }
    // 将求和送入c
    c <- sum
}

func recvChannel(c chan int) {
    for {
        v, ok := <- c
        if ok {
            fmt.Println(v)
        } else {
            // 信道已关闭
            fmt.Println("break.")
            break
        }
        time.Sleep(1000 * time.Millisecond)
    }
}

func main() {
    s := []int{7, 2, 8, -9, 4, 0}
    // 创建信道, 类型为int
    c := make(chan int)
    // 信道为带有类型的管道, 和队列类似, 用于线程间通信
    // 信道默认发送和接收在另一端准备好之前为阻塞, 所以是同步的, 不用显式加锁
    go sum(s[:len(s)/2], c)
    go sum(s[len(s)/2:], c)
    x, y := <- c, <- c    // 从chan c中接收
    sum := x + y
    fmt.Printf("x: %v, y: %v, sum: %v\n", x, y, sum)

    // 设置带缓冲的信道, 当信道满时, 发送端阻塞; 当信道为空时, 接收端阻塞
    ch := make(chan int, 3)
    go recvChannel(ch)
    ch <- 2
    ch <- 3
    ch <- 4
    // 默认当填满时再写入会报错, 存在接收端时会阻塞
    ch <- 5
    time.Sleep(5000 * time.Millisecond)
    close(ch)
    fmt.Println("close channel.")
    time.Sleep(1000 * time.Millisecond)
    // 只有发送者可以关闭信道, 关闭后的信道无法再使用
    // fmt.Println(<- ch)
    // fmt.Println(<- ch)
    // fmt.Println(<- ch)


    // 这里必须指定缓冲大小, 应该是Go的一个小bug
    ch = make(chan int, 5)
    ch <- 5
    close(ch)
    fmt.Println("close.")
    // 关闭后的信道仍然可以取得数据, 只是会非阻塞返回状态码, 所以要先判断状态码
    a, b := <- ch
    fmt.Println(a, b)
    a, b = <- ch
    fmt.Println(a, b)
}
