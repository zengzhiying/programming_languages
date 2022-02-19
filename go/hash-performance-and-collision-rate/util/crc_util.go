package util

import (
	"hash/crc32"
	"hash/crc64"
)

func CRC32(data []byte) uint32 {
	return crc32.ChecksumIEEE(data)
}

func CRC64(data []byte) uint64 {
	table := crc64.MakeTable(crc64.ECMA)
	return crc64.Checksum(data, table)
}
