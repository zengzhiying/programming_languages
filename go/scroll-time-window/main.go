package main

import (
	"fmt"
	"sync"
	"time"
)

type TextMessage struct {
	Key   int
	Value string
}

var wg sync.WaitGroup
var wg2 sync.WaitGroup

func main() {

	// custom()

	fmt.Printf("start\n")
	sw := NewScrollWindow(5)
	if err := sw.Start(); err != nil {
		panic(err)
	}

	taskNumber, batchSize := 10, 10000
	wg.Add(taskNumber)
	for i := 0; i < taskNumber; i++ {
		go Producer(batchSize, sw)
	}

	wg2.Add(1)

	go Consumer(sw)

	wg.Wait()

	fmt.Println("结束窗口.")
	sw.Stop()

	wg2.Wait()

	fmt.Println("end")
}

func Producer(n int, sw *ScrollWindow) {
	for i := 0; i < n; i++ {
		m := &TextMessage{Key: i, Value: fmt.Sprintf("number: %d", i)}
		sw.Stream <- m
		time.Sleep(time.Millisecond)
	}
	fmt.Printf("完成发送条数: %d\n", n)
	wg.Done()
}

func Consumer(sw *ScrollWindow) {
	for m := range sw.Output {
		fmt.Printf("接收到条数: %d\n", len(m))
	}
	wg2.Done()
}
