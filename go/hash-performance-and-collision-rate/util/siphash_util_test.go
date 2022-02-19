package util_test

import (
	"fmt"
	"hash-performance-and-collision-rate/util"
	"testing"
)

func TestSipHash(t *testing.T) {
	data := []byte("hello yes!")
	fmt.Println(util.SipHash64(data))

	fmt.Printf("%016x, %016x, %016x\n", util.SipHash64([]byte("1")), util.SipHash64([]byte("2")), util.SipHash64([]byte("3")))
	fmt.Printf("%016x, %016x, %016x, %016x\n", util.SipHash64([]byte("A1234566")), util.SipHash64([]byte("A1234567")), util.SipHash64([]byte("BK23R56")), util.SipHash64([]byte("A1234568")))
	fmt.Printf("%016x, %016x, %016x, %016x\n", util.SipHash64([]byte("460002708112161")), util.SipHash64([]byte("460002708112162")), util.SipHash64([]byte("460002708112163")), util.SipHash64([]byte("460002708112164")))
}
