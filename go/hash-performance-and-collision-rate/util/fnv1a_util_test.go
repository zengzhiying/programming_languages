package util_test

import (
	"fmt"
	"hash-performance-and-collision-rate/util"
	"testing"
)

func TestFNV1a(t *testing.T) {
	data := []byte("hello yes!")
	fmt.Println(util.FNV_1a64(data))

	d1, _ := util.FNV_1a64([]byte("123"))
	d2, _ := util.FNV_1a64([]byte("124"))
	d3, _ := util.FNV_1a64([]byte("125"))
	// 具有连续性
	fmt.Printf("%016x, %016x, %016x\n", d1, d2, d3)
}
