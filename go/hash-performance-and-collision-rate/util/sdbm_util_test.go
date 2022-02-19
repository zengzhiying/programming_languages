package util_test

import (
	"encoding/hex"
	"fmt"
	"hash-performance-and-collision-rate/util"
	"testing"
)

func TestSDBM(t *testing.T) {
	// data := []byte("hello yes1!")
	data1, _ := hex.DecodeString("5067e64a637ad5728741bde1599bc7b0")
	data2, _ := hex.DecodeString("e6d62d4a08924dd3c856c46720e54065")
	fmt.Println(util.SDBM(data1), util.SDBM(data2))

	data1, _ = hex.DecodeString("b4d4f47af0b08864812005cf9073fb80")
	data2, _ = hex.DecodeString("fba40466c2a76c9f459ab88a1bafe499")
	fmt.Println(util.SDBM(data1), util.SDBM(data2))
}
