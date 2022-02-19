package util

import "hash/fnv"

func FNV_1a32(data []byte) (uint32, error) {
	f := fnv.New32a()
	_, err := f.Write(data)
	return f.Sum32(), err
}

func FNV_1a64(data []byte) (uint64, error) {
	f := fnv.New64a()
	_, err := f.Write(data)
	return f.Sum64(), err
}
