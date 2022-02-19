package util

import (
	xxhash1 "github.com/OneOfOne/xxhash"
	"github.com/cespare/xxhash"
)

func XXHash64(data []byte) uint64 {
	return xxhash.Sum64(data)
}

func XXHash32(data []byte) uint32 {
	h := xxhash1.New32()
	h.Write(data)
	return h.Sum32()
}
