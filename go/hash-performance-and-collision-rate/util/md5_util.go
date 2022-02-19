package util

import "crypto/md5"

func MD5(data []byte) []byte {
	v := md5.Sum(data)
	return v[:]
}
