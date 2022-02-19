package util

func DJB2(data []byte) uint64 {
	var hash uint64 = 5381
	for _, b := range data {
		hash = ((hash << 5) + hash) + uint64(b)
		// the above line is an optimized version of the following line:
		//hash = hash * 33 + int64(c)
		// which is easier to read and understand...
	}

	return hash
}
