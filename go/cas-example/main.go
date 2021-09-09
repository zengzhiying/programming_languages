package main

import (
	"fmt"
	"sync"
	"time"
	"sync/atomic"
)

var counter uint64
var wg sync.WaitGroup
var mu sync.Mutex

func main() {
	num := 100
	wg.Add(num)

	var t1, t2 int64

	t1 = time.Now().UnixNano()
	for i := 0; i < num; i++ {
		go counterIncr(100000, 1)
	}

	wg.Wait()
	t2 = time.Now().UnixNano()
	// 900ms cpu 400%
	fmt.Printf("CAS add time: %fms, counter: %d\n", float64(t2 - t1) / 1e6, counter)

	counter = 0
	wg.Add(num)

	t1 = time.Now().UnixNano()
	for i := 0; i < num; i++ {
		go counterIncr(100000, 2)
	}

	wg.Wait()
	t2 = time.Now().UnixNano()
	// 200ms
	fmt.Printf("atomic add time: %fms, counter: %d\n", float64(t2 - t1) / 1e6, counter)

	counter = 0
	wg.Add(num)

	t1 = time.Now().UnixNano()
	for i := 0; i < num; i++ {
		go counterIncr(100000, 3)
	}

	wg.Wait()
	t2 = time.Now().UnixNano()
	// 1700ms
	fmt.Printf("lock add time: %fms, counter: %d\n", float64(t2 - t1) / 1e6, counter)

}

func counterIncr(delta, t int) {
	defer wg.Done()

	switch t {
	case 1:
		for i := 0; i < delta; i++ {
			incrCAS()
		}
	case 2:
		for i := 0; i < delta; i++ {
			incrAtomic()
		}
	case 3:
		for i := 0; i < delta; i++ {
			incrLock()
		}
	default:
		return
	}
}

func incrCAS() {
	for {
		curV := counter
		newV := curV + 1
		if atomic.CompareAndSwapUint64(&counter, curV, newV) {
			return
		}
	}
}

func incrAtomic() {
	// return new value
	atomic.AddUint64(&counter, 1)
}

func incrLock() {
	mu.Lock()
	defer mu.Unlock()
	counter++
}
