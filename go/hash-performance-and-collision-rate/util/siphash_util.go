package util

import "github.com/dchest/siphash"

func SipHash64(data []byte) uint64 {
	return siphash.Hash(0, 0, data)
}
