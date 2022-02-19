package util

func SDBM(data []byte) uint64 {
	var hash uint64 = 0

	for _, b := range data {
		hash = uint64(b) + (hash << 6) + (hash << 16) - hash
	}

	return hash
}
