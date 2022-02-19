package util_test

import (
	"encoding/hex"
	"fmt"
	"hash-performance-and-collision-rate/util"
	"testing"
)

func TestXXHash(t *testing.T) {
	// data := []byte("hello yes!")
	// fmt.Println(util.XXHash64(data))
	data1, _ := hex.DecodeString("2f001e0fde8d2c108021b66af18667f6")
	data2, _ := hex.DecodeString("f2b0576a173d62a98d766745df5b9972")
	fmt.Println(util.XXHash64(data1), util.XXHash64(data2))

	data1, _ = hex.DecodeString("a9f105e31524d8b98eddb197a8b46813")
	data2, _ = hex.DecodeString("652312eeca8b9600bed4a1b48aa5ab76")
	fmt.Println(util.XXHash64(data1), util.XXHash64(data2))

	fmt.Printf("%16x, %16x, %16x\n", util.XXHash64([]byte("1")), util.XXHash64([]byte("2")), util.XXHash64([]byte("3")))
	fmt.Printf("%016x, %016x, %016x, %016x\n", util.XXHash64([]byte("A1234566")), util.XXHash64([]byte("A1234567")), util.XXHash64([]byte("BK23R56")), util.XXHash64([]byte("A1234568")))
	fmt.Printf("%016x, %016x, %016x, %016x\n", util.XXHash64([]byte("460002708112161")), util.XXHash64([]byte("460002708112162")), util.XXHash64([]byte("460002708112163")), util.XXHash64([]byte("460002708112164")))
}
