package main

// 利用select 信道和定时器建立连续获得一定时间窗口内的数据

import (
	"fmt"
	"time"
)

var msg1 chan int
var msg2 chan string

func main() {
	msg1 = make(chan int, 10000)
	msg2 = make(chan string, 10000)
	go consumer()
	for i := 0; i < 1000; i++ {
		msg1 <- i
		time.Sleep(10*time.Millisecond)
		msg2 <- fmt.Sprintf("Hello %d.", i)
		time.Sleep(90*time.Millisecond)
	}

	time.Sleep(1000*time.Second)
}


func consumer() {
	var i int
	var s string
	msgType := 0
	number := 0
	total := 0
	ticker := time.NewTicker(time.Second * 2)
	for {
		<-ticker.C
		fmt.Println("C")
		for {
			select {
			case i = <- msg1:
				msgType = 1
				number++
			case s = <- msg2:
				msgType = 2
				number++
			default:
				fmt.Println("msg empty! number: ", number)
				number = 0
				msgType = 0
			}
			if msgType == 1 {
				fmt.Println("Consumerd:", i)
				total++
			} else if msgType == 2 {
				fmt.Println("Consumerd: ", s)
				total++
			} else {
				break
			}
		}
		fmt.Printf("total: %d\n", total)
	}
}