package main

import (
	"fmt"
	"sync"
	"time"
)

type any = interface{}

const bufferSize = 10000000
const resultSize = 100

type ScrollWindow struct {
	Second int
	Stream chan any
	Output chan []any
	// Stream   reflect.Value
	// Output   reflect.Value
	started bool
	ticker  *time.Ticker
	lock    sync.Mutex
}

func NewScrollWindow(second int) *ScrollWindow {
	swIn := make(chan any, bufferSize)
	swOut := make(chan []any, resultSize)
	// swIn := reflect.MakeChan(reflect.ChanOf(reflect.BothDir, reflect.TypeOf(t)), bufferSize)
	// swOut := reflect.MakeChan(reflect.ChanOf(reflect.BothDir, reflect.ArrayOf(bufferSize, reflect.TypeOf(t))), bufferSize)

	return &ScrollWindow{
		Second:  second,
		Stream:  swIn,
		Output:  swOut,
		started: false,
		ticker:  time.NewTicker(time.Duration(second) * time.Second),
	}
}

func (sw *ScrollWindow) Start() error {
	sw.lock.Lock()
	defer sw.lock.Unlock()

	if sw.started {
		return fmt.Errorf("has been activated")
	}

	sw.started = true
	go sw.start()
	return nil
}

func (sw *ScrollWindow) start() {
	for range sw.ticker.C {
		r := make([]any, 0)
		end := false
		for !end {
			select {
			case s := <-sw.Stream:
				r = append(r, s)
			default:
				end = true
			}
		}

		sw.Output <- r
	}
}

func (sw *ScrollWindow) Stop() error {
	sw.lock.Lock()
	defer sw.lock.Unlock()
	if !sw.started {
		return fmt.Errorf("has stopped")
	}
	sw.ticker.Stop()

	close(sw.Stream)

	r := make([]any, 0)
	for s := range sw.Stream {
		r = append(r, s)
	}

	sw.Output <- r

	close(sw.Output)

	sw.started = false
	return nil
}
