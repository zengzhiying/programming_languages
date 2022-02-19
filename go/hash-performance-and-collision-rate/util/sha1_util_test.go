package util_test

import (
	"encoding/hex"
	"fmt"
	"hash-performance-and-collision-rate/util"
	"testing"
)

func TestSHA1(t *testing.T) {
	data := []byte("hello yes!")
	fmt.Println(hex.EncodeToString(util.SHA1(data)))
}
