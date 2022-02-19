package util

import "crypto/sha1"

func SHA1(data []byte) []byte {
	v := sha1.Sum(data)
	return v[:]
}
