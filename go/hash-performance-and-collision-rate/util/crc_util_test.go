package util_test

import (
	"fmt"
	"hash-performance-and-collision-rate/util"
	"testing"
)

func TestCRC32(t *testing.T) {
	data := []byte("hello yes!")
	fmt.Printf("crc32: %d\n", util.CRC32(data))
}

func TestCRC64(t *testing.T) {
	data := []byte("hello yes!")
	fmt.Printf("crc64: %d\n", util.CRC64(data))
}
