package main

import (
	"fmt"
	"sync"
	"time"

	ants "github.com/panjf2000/ants/v2"
	"github.com/shettyh/threadpool"
)

// 当poolSize设置为10000, numberTask设置为100000时
// Intel Silver 4114 * 2 (20核 40线程)  thread pool: 4505ms   ants: 3955ms
// 云虚拟机 8核  thread pool: 16727ms  ants: 12949ms
// 当任务少的时候在高配机器上两个库几乎没有差别, 随着任务量的增大 ants性能优势逐渐明显, 所以对于比较重的并发任务建议采用ants库

const numberTask = 100000
const poolSize = 10000

var wg sync.WaitGroup

func main() {
	var t1, t2 int64
	var delta float64
	// custom pool
	t1 = time.Now().UnixNano()
	cutsomBenchmark()
	t2 = time.Now().UnixNano()
	delta = float64(t2-t1) / 1e6
	fmt.Printf("Custom pool benchmark: %.3fms\n", delta)

	// thread pool
	t1 = time.Now().UnixNano()
	threadPoolBenchmark()
	t2 = time.Now().UnixNano()
	delta = float64(t2-t1) / 1e6
	fmt.Printf("Thread pool benchmark: %.3fms\n", delta)

	// ants pool
	t1 = time.Now().UnixNano()
	antsBenchmark()
	t2 = time.Now().UnixNano()
	delta = float64(t2-t1) / 1e6
	fmt.Printf("ants benchmark: %.3fms\n", delta)

}

type MyTask struct {
	Count int
}

func (t *MyTask) Run() {
	for i := 0; i < 1000000; i += 2 {
		t.Count += i
	}
	wg.Done()
}

type FuncPool struct {
	ch chan int
}

func NewFuncPool(size int) *FuncPool {
	return &FuncPool{ch: make(chan int, size)}
}

func (pool *FuncPool) Execute(ff func()) {
	pool.ch <- 1
	defer func() {
		if r := recover(); r != nil {
			fmt.Printf("Recover: %s\n", r)
		}
		<-pool.ch
	}()

	go ff()
}

func cutsomBenchmark() {
	pool := NewFuncPool(poolSize)
	tasks := make([]MyTask, numberTask)
	for i := 0; i < numberTask; i++ {
		tasks[i] = MyTask{Count: i}
		wg.Add(1)
	}

	for i := 0; i < numberTask; i++ {
		task := &tasks[i]
		pool.Execute(task.Run)
	}

	wg.Wait()

	total := 0
	for _, task := range tasks {
		total += task.Count
	}
	fmt.Printf("Total: %d\n", total)
}

func threadPoolBenchmark() {
	pool := threadpool.NewThreadPool(poolSize, numberTask)
	tasks := make([]MyTask, numberTask)
	for i := 0; i < numberTask; i++ {
		tasks[i] = MyTask{Count: i}
		wg.Add(1)
	}
	for i := 0; i < numberTask; i++ {
		task := &tasks[i]
		err := pool.Execute(task)
		if err != nil {
			fmt.Printf("Pool execute err: %s\n", err)
			wg.Done()
		}
	}

	wg.Wait()
	total := 0
	for _, task := range tasks {
		total += task.Count
	}
	fmt.Printf("Total: %d\n", total)
}

func antsBenchmark() {
	pool, err := ants.NewPoolWithFunc(poolSize, func(i interface{}) {
		i.(*MyTask).Run()
	})
	if err != nil {
		fmt.Printf("New Pool err: %s\n", err)
	}

	defer pool.Release()

	tasks := make([]MyTask, numberTask)
	for i := 0; i < numberTask; i++ {
		tasks[i] = MyTask{Count: i}
		wg.Add(1)
	}

	for i := 0; i < numberTask; i++ {
		task := &tasks[i]
		err := pool.Invoke(task)
		if err != nil {
			fmt.Printf("Poll invoke err: %s\n", err)
		}
	}

	wg.Wait()
	total := 0
	for _, task := range tasks {
		total += task.Count
	}
	fmt.Printf("Total: %d\n", total)

}
