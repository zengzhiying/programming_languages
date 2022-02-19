package util_test

import (
	"fmt"
	"hash-performance-and-collision-rate/util"
	"testing"
)

func TestDJB2(t *testing.T) {
	data := []byte("hello world, 世界")
	fmt.Println(util.DJB2(data) & 0xffffffff)

	// 具有连续性
	fmt.Printf("%016x, %016x, %016x\n", util.DJB2([]byte("123")), util.DJB2([]byte("124")), util.DJB2([]byte("125")))
	fmt.Printf("%016x, %016x, %016x, %016x\n", util.DJB2([]byte("A1234566")), util.DJB2([]byte("A1234567")), util.DJB2([]byte("BK23R56")), util.DJB2([]byte("A1234568")))
	fmt.Printf("%016x, %016x, %016x, %016x\n", util.DJB2([]byte("460002708112161")), util.DJB2([]byte("460002708112162")), util.DJB2([]byte("460002708112163")), util.DJB2([]byte("460002708112164")))
}
