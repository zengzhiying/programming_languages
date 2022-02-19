package util_test

import (
	"encoding/hex"
	"fmt"
	"hash-performance-and-collision-rate/util"
	"testing"
)

func TestMD5(t *testing.T) {
	data := []byte("hello yes!")
	fmt.Println(hex.EncodeToString(util.MD5(data)))
}
